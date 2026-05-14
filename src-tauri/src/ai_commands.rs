use crate::domain::{AiContext, AiResult, Command, Method, JobType, Basis, Solvent, AtomSummary, CalculationSummary, atom_index, atom_position};
use crate::gaussian::method_name;
use crate::geometry::{dihedral_degrees, sub, rotate};

pub fn build_ai_context(state: &crate::domain::AppState, screenshot: Option<String>) -> AiContext {
    let molecule = &state.domain.chemical_spec.molecule;
    let calculation = &state.domain.chemical_spec.calculation;
    let selected_atoms = state
        .ui
        .selected_atoms
        .iter()
        .filter_map(|atom_id| molecule.atoms.iter().find(|atom| atom.id == *atom_id))
        .map(|atom| AtomSummary {
            id: atom.id,
            element: atom.element,
            isotope: atom.isotope,
            nuclear_spin: atom.nuclear_spin,
            position: atom.position,
        })
        .collect::<Vec<_>>();

    AiContext {
        selected_atoms,
        calculation: CalculationSummary {
            job_type: calculation.job_type,
            method: calculation.method,
            basis: calculation.basis,
            solvent: calculation.solvent,
            charge: calculation.charge,
            multiplicity: calculation.multiplicity,
        },
        screenshot,
    }
}

pub fn propose_ai_commands(input: &str, context: &AiContext) -> AiResult {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return AiResult {
            commands: Vec::new(),
            explanation: "No request was provided.".to_string(),
        };
    }

    if let Some(result) = parse_json_ai_result(trimmed) {
        return result;
    }

    let normalized = trimmed.to_ascii_lowercase();
    let mut commands = Vec::new();

    if normalized.contains("b3lyp") {
        commands.push(Command::SetMethod {
            method: Method::B3LYP,
        });
    }
    if normalized.contains("wb97xd") {
        commands.push(Command::SetMethod {
            method: Method::WB97XD,
        });
    }

    if normalized.contains("6-31g(d)") {
        commands.push(Command::SetBasis {
            basis: Basis::Six31Gd,
        });
    }
    if normalized.contains("def2-svp") {
        commands.push(Command::SetBasis {
            basis: Basis::Def2Svp,
        });
    }
    if normalized.contains("def2-tzvp") {
        commands.push(Command::SetBasis {
            basis: Basis::Def2Tzvp,
        });
    }

    if normalized.contains("thf") {
        commands.push(Command::SetSolvent {
            solvent: Some(Solvent::THF),
        });
    }
    if normalized.contains("water") {
        commands.push(Command::SetSolvent {
            solvent: Some(Solvent::Water),
        });
    }
    if normalized.contains("no solvent") || normalized.contains("gas phase") {
        commands.push(Command::SetSolvent { solvent: None });
    }

    if let Some(job_type) = infer_job_type(&normalized) {
        commands.push(Command::SetJobType { job_type });
    }
    if let Some(charge) = parse_number_after(&normalized, "charge") {
        commands.push(Command::SetCharge { charge });
    }
    if let Some(multiplicity) = parse_number_after(&normalized, "multiplicity")
        .or_else(|| parse_number_after(&normalized, "mult"))
        .and_then(|value| u32::try_from(value).ok())
    {
        commands.push(Command::SetMultiplicity { multiplicity });
    }
    if let Some(command) = infer_geometry_command(&normalized, context) {
        commands.push(command);
    }

    let unique_commands = dedupe_ai_commands(commands);
    let explanation = if unique_commands.is_empty() {
        "No supported changes were found. Try mentioning method, basis, job type, solvent, charge, multiplicity, bond length, bond angle, or dihedral angle."
            .to_string()
    } else {
        format!(
            "Proposed {} command(s) from the request. Current method is {}.",
            unique_commands.len(),
            method_name(context.calculation.method)
        )
    };

    AiResult {
        commands: unique_commands,
        explanation,
    }
}

pub fn parse_ai_result_json(text: &str) -> Result<AiResult, String> {
    let parsed = serde_json::from_str::<AiResult>(text).map_err(|error| error.to_string())?;
    let commands = parsed
        .commands
        .into_iter()
        .filter(is_ai_command)
        .collect::<Vec<_>>();
    Ok(AiResult {
        commands,
        explanation: if parsed.explanation.is_empty() {
            "Parsed JSON commands.".to_string()
        } else {
            parsed.explanation
        },
    })
}

fn parse_json_ai_result(text: &str) -> Option<AiResult> {
    parse_ai_result_json(text).ok()
}

fn is_ai_command(command: &Command) -> bool {
    matches!(
        command,
        Command::SetMethod { .. }
            | Command::SetBasis { .. }
            | Command::SetJobType { .. }
            | Command::SetSolvent { .. }
            | Command::SetCharge { .. }
            | Command::SetMultiplicity { .. }
            | Command::SetBondLength { .. }
            | Command::SetBondAngle { .. }
            | Command::SetDihedralAngle { .. }
            | Command::AddAtom { .. }
            | Command::DeleteAtom { .. }
            | Command::AddBond { .. }
            | Command::DeleteBond { .. }
    )
}

fn infer_job_type(text: &str) -> Option<JobType> {
    if text.contains("transition state")
        || text.split_whitespace().any(|token| token == "ts")
    {
        return Some(JobType::Ts);
    }

    let has_opt = text.contains("opt")
        || text.contains("optimize")
        || text.contains("optimization");
    let has_freq = text.contains("freq") || text.contains("frequency");
    match (has_opt, has_freq) {
        (true, true) => Some(JobType::OptFreq),
        (true, false) => Some(JobType::Opt),
        (false, true) => Some(JobType::Freq),
        (false, false) => None,
    }
}

fn parse_number_after(text: &str, keyword: &str) -> Option<i32> {
    let words = text.split_whitespace().collect::<Vec<_>>();
    for (index, word) in words.iter().enumerate() {
        if *word == keyword {
            let next = words.get(index + 1)?;
            let numeric = next.trim_matches(|char: char| {
                char == ':' || char == '=' || char == ','
            });
            if let Ok(value) = numeric.parse::<i32>() {
                return Some(value);
            }
        }

        if let Some(rest) = word.strip_prefix(keyword) {
            let numeric = rest.trim_matches(|char: char| {
                char == ':' || char == '=' || char == ','
            });
            if !numeric.is_empty() {
                if let Ok(value) = numeric.parse::<i32>() {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn infer_geometry_command(text: &str, context: &AiContext) -> Option<Command> {
    let value = parse_geometry_value(text)?;
    let selected = context
        .selected_atoms
        .iter()
        .map(|atom| atom.id)
        .collect::<Vec<_>>();

    if (text.contains("dihedral") || text.contains("torsion"))
        && selected.len() >= 4
    {
        return Some(Command::SetDihedralAngle {
            atom_ids: [selected[0], selected[1], selected[2], selected[3]],
            angle: value,
        });
    }
    if (text.contains("bond angle") || text.contains("angle"))
        && selected.len() >= 3
    {
        return Some(Command::SetBondAngle {
            atom_ids: [selected[0], selected[1], selected[2]],
            angle: value,
        });
    }
    if (text.contains("bond length") || text.contains("distance"))
        && selected.len() >= 2
    {
        return Some(Command::SetBondLength {
            atom_ids: [selected[0], selected[1]],
            length: value,
        });
    }

    None
}

fn parse_geometry_value(text: &str) -> Option<f64> {
    text.split(|char: char| {
        matches!(
            char,
            ' ' | '　' | ':' | '=' | ',' | ';' | '(' | ')' | '[' | ']'
        )
    })
    .filter_map(|part| {
        let trimmed = part.trim_matches(|char: char| {
            matches!(char, 'a' | 'A' | '°' | 'Å' | 'Å')
        });
        if trimmed.is_empty() {
            return None;
        }
        // Handle cases like "1.42オングストローム"
        let numeric_part = if let Some(index) = trimmed.find(|c: char| !c.is_ascii_digit() && c != '.') {
            &trimmed[..index]
        } else {
            trimmed
        };
        numeric_part.parse::<f64>().ok()
    })
    .last()
}

fn dedupe_ai_commands(commands: Vec<Command>) -> Vec<Command> {
    let mut unique = Vec::new();
    let mut method = None;
    let mut basis = None;
    let mut job_type = None;
    let mut solvent = None;
    let mut charge = None;
    let mut multiplicity = None;

    for command in commands {
        match command {
            Command::SetMethod { .. } => method = Some(command),
            Command::SetBasis { .. } => basis = Some(command),
            Command::SetJobType { .. } => job_type = Some(command),
            Command::SetSolvent { .. } => solvent = Some(command),
            Command::SetCharge { .. } => charge = Some(command),
            Command::SetMultiplicity { .. } => multiplicity = Some(command),
            Command::SetBondLength { .. }
            | Command::SetBondAngle { .. }
            | Command::SetDihedralAngle { .. }
            | Command::AddAtom { .. }
            | Command::DeleteAtom { .. }
            | Command::AddBond { .. }
            | Command::DeleteBond { .. } => unique.push(command),
            Command::SetMolecule { .. }
            | Command::ToggleAtomSelection { .. }
            | Command::ClearSelection => {}
        }
    }

    unique.extend(
        [method, basis, job_type, solvent, charge, multiplicity]
            .into_iter()
            .flatten(),
    );
    unique
}
