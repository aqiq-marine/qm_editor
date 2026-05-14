use rig::{completion::Prompt, prelude::*, providers::gemini};
use serde::Serialize;

use crate::domain::{self, AiContext, AiResult, AppState};

const DEFAULT_GEMINI_MODEL: &str = "gemini-2.5-flash-lite";

#[derive(Clone, Copy, Debug)]
enum AiProvider {
    GoogleGemini,
}

impl AiProvider {
    fn from_env() -> Result<Self, String> {
        match std::env::var("QM_EDITOR_AI_PROVIDER")
            .unwrap_or_else(|_| "google-gemini".to_string())
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "" | "google" | "google-gemini" | "gemini" => Ok(Self::GoogleGemini),
            provider => Err(format!("Unsupported AI provider '{provider}'.")),
        }
    }
}

pub async fn propose_ai_commands(
    input: &str,
    state: &AppState,
    context: &AiContext,
) -> Result<AiResult, String> {
    if input.trim().is_empty() {
        return Ok(domain::propose_ai_commands(input, context));
    }

    if let Ok(result) = domain::parse_ai_result_json(input.trim()) {
        return Ok(result);
    }

    if let Some(result) = local_result_for_supported_request(input, context) {
        return Ok(result);
    }

    match AiProvider::from_env()? {
        AiProvider::GoogleGemini => propose_with_gemini(input, state, context).await,
    }
}

fn local_result_for_supported_request(input: &str, context: &AiContext) -> Option<AiResult> {
    let result = domain::propose_ai_commands(input, context);
    (!result.commands.is_empty()).then_some(result)
}

async fn propose_with_gemini(
    input: &str,
    state: &AppState,
    context: &AiContext,
) -> Result<AiResult, String> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .or_else(|_| std::env::var("GOOGLE_API_KEY"))
        .map_err(|_| "Set GEMINI_API_KEY or GOOGLE_API_KEY to use the AI assistant.".to_string())?;
    let client = gemini::Client::new(api_key).map_err(|error| error.to_string())?;
    let model = std::env::var("QM_EDITOR_GEMINI_MODEL")
        .unwrap_or_else(|_| DEFAULT_GEMINI_MODEL.to_string());
    let agent = client
        .agent(model)
        .preamble(system_prompt())
        .temperature(0.0)
        .build();
    let prompt = build_prompt(input, state, context)?;
    let response = agent
        .prompt(prompt)
        .await
        .map_err(|error| error.to_string())?;
    let json = extract_json_object(&response)
        .ok_or_else(|| "AI response did not contain a JSON object.".to_string())?;

    domain::parse_ai_result_json(json)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PromptPayload<'a> {
    request: &'a str,
    state: &'a AppState,
    context: &'a AiContext,
}

fn build_prompt(input: &str, state: &AppState, context: &AiContext) -> Result<String, String> {
    let payload = PromptPayload {
        request: input,
        state,
        context,
    };
    serde_json::to_string_pretty(&payload).map_err(|error| error.to_string())
}

fn system_prompt() -> &'static str {
    r#"You convert user requests for a molecular Gaussian input editor into JSON commands.
Return only one JSON object with this exact shape:
{"commands":[],"explanation":"short explanation"}

Allowed command type values:
SET_METHOD, SET_BASIS, SET_JOB_TYPE, SET_SOLVENT, SET_CHARGE, SET_MULTIPLICITY,
SET_BOND_LENGTH, SET_BOND_ANGLE, SET_DIHEDRAL_ANGLE, ADD_ATOM, DELETE_ATOM, ADD_BOND, DELETE_BOND.

Use only values supported by the provided state:
method: B3LYP or WB97XD
basis: 6-31G(d), def2-SVP, or def2-TZVP
jobType: opt, freq, opt+freq, or ts
solvent: THF, Water, or null

Use camelCase fields exactly as shown in the state. Never include SET_MOLECULE,
TOGGLE_ATOM_SELECTION, CLEAR_SELECTION, markdown, comments, or extra text."#
}

fn extract_json_object(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        return Some(trimmed);
    }

    let start = trimmed.find('{')?;
    let end = trimmed.rfind('}')?;
    (start < end).then(|| &trimmed[start..=end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{self, Command, Method};

    #[test]
    fn uses_local_parser_for_supported_short_request() {
        let state = domain::initial_app_state();
        let context = domain::build_ai_context(&state, None);
        let result = local_result_for_supported_request("set wb97xd", &context)
            .expect("supported short request should be handled locally");

        assert!(matches!(
            result.commands.as_slice(),
            [Command::SetMethod {
                method: Method::WB97XD
            }]
        ));
    }
}
