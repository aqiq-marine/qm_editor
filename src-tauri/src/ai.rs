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

Allowed command types and their fields:
- SET_METHOD: {"type": "SET_METHOD", "method": "B3LYP" | "WB97XD"}
- SET_BASIS: {"type": "SET_BASIS", "basis": "6-31G(d)" | "def2-SVP" | "def2-TZVP"}
- SET_JOB_TYPE: {"type": "SET_JOB_TYPE", "jobType": "opt" | "freq" | "opt+freq" | "ts"}
- SET_SOLVENT: {"type": "SET_SOLVENT", "solvent": "THF" | "Water" | null}
- SET_CHARGE: {"type": "SET_CHARGE", "charge": number}
- SET_MULTIPLICITY: {"type": "SET_MULTIPLICITY", "multiplicity": number}
- SET_BOND_LENGTH: {"type": "SET_BOND_LENGTH", "atomIds": [id1, id2], "length": number}
- SET_BOND_ANGLE: {"type": "SET_BOND_ANGLE", "atomIds": [id1, id2, id3], "angle": number}
- SET_DIHEDRAL_ANGLE: {"type": "SET_DIHEDRAL_ANGLE", "atomIds": [id1, id2, id3, id4], "angle": number}
- ADD_ATOM: {"type": "ADD_ATOM", "element": string, "position": [x, y, z], "isotope"?: number, "nuclearSpin"?: number}
- DELETE_ATOM: {"type": "DELETE_ATOM", "atomId": number}
- ADD_BOND: {"type": "ADD_BOND", "atomIds": [id1, id2], "order": 1 | 2 | 3}
- DELETE_BOND: {"type": "DELETE_BOND", "bondId": number}

Use camelCase fields exactly as shown. For geometry changes (length, angle, dihedral), use IDs from the provided selectedAtoms if they match the required count. Never include markdown, comments, or extra text."#
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
