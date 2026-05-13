use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub domain: DomainState,
    pub ui: UiState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainState {
    pub chemical_spec: ChemicalSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    pub selected_atoms: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChemicalSpec {
    pub molecule: Molecule,
    pub calculation: CalculationSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Molecule {
    pub name: String,
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atom {
    pub id: u32,
    pub element: String,
    pub position: [f64; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bond {
    pub id: u32,
    pub atom_ids: [u32; 2],
    pub order: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculationSpec {
    pub job_type: JobType,
    pub method: Method,
    pub basis: Basis,
    pub solvent: Option<Solvent>,
    pub charge: i32,
    pub multiplicity: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JobType {
    #[serde(rename = "opt")]
    Opt,
    #[serde(rename = "freq")]
    Freq,
    #[serde(rename = "opt+freq")]
    OptFreq,
    #[serde(rename = "ts")]
    Ts,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Method {
    B3LYP,
    WB97XD,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Basis {
    #[serde(rename = "6-31G(d)")]
    Six31Gd,
    #[serde(rename = "def2-SVP")]
    Def2Svp,
    #[serde(rename = "def2-TZVP")]
    Def2Tzvp,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Solvent {
    THF,
    Water,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", rename_all_fields = "camelCase")]
pub enum Command {
    SetMethod { method: Method },
    SetBasis { basis: Basis },
    SetJobType { job_type: JobType },
    SetSolvent { solvent: Option<Solvent> },
    SetCharge { charge: i32 },
    SetMultiplicity { multiplicity: u32 },
    SetMolecule { molecule: Molecule },
    ToggleAtomSelection { atom_id: u32 },
    ClearSelection,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationMessage {
    pub level: ValidationLevel,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationLevel {
    Error,
    Warning,
}

pub fn initial_app_state() -> AppState {
    AppState {
        domain: DomainState {
            chemical_spec: ChemicalSpec {
                molecule: Molecule {
                    name: "Water".to_string(),
                    atoms: vec![
                        Atom {
                            id: 1,
                            element: "O".to_string(),
                            position: [0.0, 0.0, 0.0],
                        },
                        Atom {
                            id: 2,
                            element: "H".to_string(),
                            position: [0.758, 0.586, 0.0],
                        },
                        Atom {
                            id: 3,
                            element: "H".to_string(),
                            position: [-0.758, 0.586, 0.0],
                        },
                    ],
                    bonds: vec![
                        Bond {
                            id: 1,
                            atom_ids: [1, 2],
                            order: 1,
                        },
                        Bond {
                            id: 2,
                            atom_ids: [1, 3],
                            order: 1,
                        },
                    ],
                },
                calculation: CalculationSpec {
                    job_type: JobType::Opt,
                    method: Method::B3LYP,
                    basis: Basis::Six31Gd,
                    solvent: None,
                    charge: 0,
                    multiplicity: 1,
                },
            },
        },
        ui: UiState {
            selected_atoms: Vec::new(),
        },
    }
}

pub fn reduce(mut state: AppState, command: Command) -> AppState {
    match command {
        Command::SetMethod { method } => state.domain.chemical_spec.calculation.method = method,
        Command::SetBasis { basis } => state.domain.chemical_spec.calculation.basis = basis,
        Command::SetJobType { job_type } => state.domain.chemical_spec.calculation.job_type = job_type,
        Command::SetSolvent { solvent } => state.domain.chemical_spec.calculation.solvent = solvent,
        Command::SetCharge { charge } => state.domain.chemical_spec.calculation.charge = charge,
        Command::SetMultiplicity { multiplicity } => {
            state.domain.chemical_spec.calculation.multiplicity = multiplicity
        }
        Command::SetMolecule { molecule } => {
            state.domain.chemical_spec.molecule = molecule;
            state.ui.selected_atoms.clear();
        }
        Command::ToggleAtomSelection { atom_id } => {
            if let Some(index) = state.ui.selected_atoms.iter().position(|id| *id == atom_id) {
                state.ui.selected_atoms.remove(index);
            } else {
                state.ui.selected_atoms.push(atom_id);
            }
        }
        Command::ClearSelection => state.ui.selected_atoms.clear(),
    }
    state
}

pub fn parse_molecule_file(file_name: &str, text: &str) -> Result<Molecule, String> {
    match file_name.rsplit('.').next().map(str::to_ascii_lowercase) {
        Some(extension) if extension == "xyz" => parse_xyz(file_name, text),
        Some(extension) if extension == "mol" => parse_mol(file_name, text),
        _ => Err("Unsupported molecule file. Import .xyz or .mol.".to_string()),
    }
}

pub fn render_gaussian(spec: &ChemicalSpec) -> String {
    let calculation = &spec.calculation;
    let molecule = &spec.molecule;
    let mut route = vec![
        route_job(calculation.job_type).to_string(),
        format!("{}/{}", method_name(calculation.method), basis_name(calculation.basis)),
    ];
    if let Some(solvent) = calculation.solvent {
        route.push(format!("SCRF=(Solvent={})", solvent_name(solvent)));
    }

    let coordinates = molecule
        .atoms
        .iter()
        .map(|atom| {
            format!(
                "{:<2} {:>12.6} {:>12.6} {:>12.6}",
                atom.element, atom.position[0], atom.position[1], atom.position[2]
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "%chk={}.chk\n# {}\n\n{}\n\n{} {}\n{}\n\n",
        safe_name(&molecule.name),
        route.join(" "),
        if molecule.name.is_empty() {
            "Gaussian input"
        } else {
            &molecule.name
        },
        calculation.charge,
        calculation.multiplicity,
        coordinates
    )
}

pub fn validate_chemical_spec(spec: &ChemicalSpec) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();
    let molecule = &spec.molecule;
    let calculation = &spec.calculation;

    if molecule.atoms.is_empty() {
        messages.push(error("Molecule must contain at least one atom."));
    }

    if calculation.multiplicity < 1 {
        messages.push(error("Multiplicity must be a positive integer."));
    }

    let charge_parity = calculation.charge.unsigned_abs() % 2;
    let electron_parity = molecule.atoms.iter().fold(charge_parity, |parity, atom| {
        (parity + valence_parity(&atom.element)) % 2
    });
    let unpaired_parity = (calculation.multiplicity - 1) % 2;
    if !molecule.atoms.is_empty() && electron_parity != unpaired_parity {
        messages.push(warning(
            "Charge and multiplicity look inconsistent for common valence parity.",
        ));
    }

    messages
}

fn parse_xyz(file_name: &str, text: &str) -> Result<Molecule, String> {
    let lines = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let atom_count = lines
        .first()
        .ok_or_else(|| "XYZ file must start with an atom count.".to_string())?
        .parse::<usize>()
        .map_err(|_| "XYZ file must start with an atom count.".to_string())?;

    let mut atoms = Vec::with_capacity(atom_count);
    for (index, line) in lines.iter().skip(2).take(atom_count).enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 4 {
            return Err(format!("Invalid XYZ atom line {}.", index + 3));
        }
        atoms.push(Atom {
            id: (index + 1) as u32,
            element: normalize_element(parts[0]),
            position: [
                parse_coord(parts[1], "XYZ coordinates must be numeric.")?,
                parse_coord(parts[2], "XYZ coordinates must be numeric.")?,
                parse_coord(parts[3], "XYZ coordinates must be numeric.")?,
            ],
        });
    }

    if atoms.len() != atom_count {
        return Err("XYZ file ended before all atoms were read.".to_string());
    }

    Ok(Molecule {
        name: lines
            .get(1)
            .map(|line| line.to_string())
            .filter(|line| !line.is_empty())
            .unwrap_or_else(|| strip_extension(file_name)),
        bonds: infer_bonds(&atoms),
        atoms,
    })
}

fn parse_mol(file_name: &str, text: &str) -> Result<Molecule, String> {
    let lines = text.lines().collect::<Vec<_>>();
    let counts = lines
        .get(3)
        .ok_or_else(|| "MOL file is missing a counts line.".to_string())?;
    let atom_count = counts
        .get(0..3)
        .unwrap_or("")
        .trim()
        .parse::<usize>()
        .map_err(|_| "MOL counts line is invalid.".to_string())?;
    let bond_count = counts
        .get(3..6)
        .unwrap_or("")
        .trim()
        .parse::<usize>()
        .map_err(|_| "MOL counts line is invalid.".to_string())?;

    let mut atoms = Vec::with_capacity(atom_count);
    for (index, line) in lines.iter().skip(4).take(atom_count).enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 4 {
            return Err(format!("Invalid MOL atom line {}.", index + 5));
        }
        atoms.push(Atom {
            id: (index + 1) as u32,
            element: normalize_element(parts[3]),
            position: [
                parse_coord(parts[0], "MOL coordinates must be numeric.")?,
                parse_coord(parts[1], "MOL coordinates must be numeric.")?,
                parse_coord(parts[2], "MOL coordinates must be numeric.")?,
            ],
        });
    }

    let mut bonds = Vec::with_capacity(bond_count);
    for (index, line) in lines.iter().skip(4 + atom_count).take(bond_count).enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 3 {
            return Err(format!("Invalid MOL bond line {}.", index + atom_count + 5));
        }
        let first = parts[0]
            .parse::<u32>()
            .map_err(|_| format!("Invalid MOL bond line {}.", index + atom_count + 5))?;
        let second = parts[1]
            .parse::<u32>()
            .map_err(|_| format!("Invalid MOL bond line {}.", index + atom_count + 5))?;
        let order = parts[2].parse::<u8>().unwrap_or(1).clamp(1, 3);
        bonds.push(Bond {
            id: (index + 1) as u32,
            atom_ids: [first, second],
            order,
        });
    }

    Ok(Molecule {
        name: lines
            .first()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .unwrap_or_else(|| strip_extension(file_name)),
        atoms,
        bonds,
    })
}

fn infer_bonds(atoms: &[Atom]) -> Vec<Bond> {
    let mut bonds = Vec::new();
    for first_index in 0..atoms.len() {
        for second_index in (first_index + 1)..atoms.len() {
            let first = &atoms[first_index];
            let second = &atoms[second_index];
            let threshold = covalent_radius(&first.element) + covalent_radius(&second.element) + 0.45;
            if distance(first.position, second.position) <= threshold {
                bonds.push(Bond {
                    id: (bonds.len() + 1) as u32,
                    atom_ids: [first.id, second.id],
                    order: 1,
                });
            }
        }
    }
    bonds
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn covalent_radius(element: &str) -> f64 {
    match element {
        "H" => 0.31,
        "C" => 0.76,
        "N" => 0.71,
        "O" => 0.66,
        "F" => 0.57,
        "P" => 1.07,
        "S" => 1.05,
        "Cl" => 1.02,
        "Br" => 1.20,
        "I" => 1.39,
        _ => 0.75,
    }
}

fn valence_parity(element: &str) -> u32 {
    match element {
        "H" | "B" | "N" | "F" | "P" | "Cl" | "Br" | "I" => 1,
        _ => 0,
    }
}

fn route_job(job_type: JobType) -> &'static str {
    match job_type {
        JobType::Opt => "Opt",
        JobType::Freq => "Freq",
        JobType::OptFreq => "Opt Freq",
        JobType::Ts => "Opt=(TS,CalcFC,NoEigenTest)",
    }
}

fn method_name(method: Method) -> &'static str {
    match method {
        Method::B3LYP => "B3LYP",
        Method::WB97XD => "WB97XD",
    }
}

fn basis_name(basis: Basis) -> &'static str {
    match basis {
        Basis::Six31Gd => "6-31G(d)",
        Basis::Def2Svp => "def2-SVP",
        Basis::Def2Tzvp => "def2-TZVP",
    }
}

fn solvent_name(solvent: Solvent) -> &'static str {
    match solvent {
        Solvent::THF => "THF",
        Solvent::Water => "Water",
    }
}

fn safe_name(name: &str) -> String {
    let normalized = name
        .trim()
        .chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() || char == '_' || char == '-' {
                char
            } else {
                '_'
            }
        })
        .collect::<String>();
    if normalized.is_empty() {
        "molecule".to_string()
    } else {
        normalized
    }
}

fn normalize_element(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => {
            let rest = chars.as_str().to_ascii_lowercase();
            format!("{}{}", first.to_ascii_uppercase(), rest)
        }
        None => String::new(),
    }
}

fn parse_coord(value: &str, message: &str) -> Result<f64, String> {
    value.parse::<f64>().map_err(|_| message.to_string())
}

fn strip_extension(file_name: &str) -> String {
    file_name
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(file_name)
        .to_string()
}

fn error(message: &str) -> ValidationMessage {
    ValidationMessage {
        level: ValidationLevel::Error,
        message: message.to_string(),
    }
}

fn warning(message: &str) -> ValidationMessage {
    ValidationMessage {
        level: ValidationLevel::Warning,
        message: message.to_string(),
    }
}
