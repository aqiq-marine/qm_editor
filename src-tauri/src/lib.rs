mod domain;

use domain::{AppState, ChemicalSpec, Command, Molecule, ValidationMessage};

#[tauri::command]
fn get_initial_app_state() -> AppState {
    domain::initial_app_state()
}

#[tauri::command]
fn apply_command(state: AppState, command: Command) -> AppState {
    domain::reduce(state, command)
}

#[tauri::command]
fn apply_commands(state: AppState, commands: Vec<Command>) -> AppState {
    commands
        .into_iter()
        .fold(state, |current_state, command| domain::reduce(current_state, command))
}

#[tauri::command]
fn parse_molecule_file(file_name: String, text: String) -> Result<Molecule, String> {
    domain::parse_molecule_file(&file_name, &text)
}

#[tauri::command]
fn render_gaussian(spec: ChemicalSpec) -> String {
    domain::render_gaussian(&spec)
}

#[tauri::command]
fn validate_chemical_spec(spec: ChemicalSpec) -> Vec<ValidationMessage> {
    domain::validate_chemical_spec(&spec)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_initial_app_state,
            apply_command,
            apply_commands,
            parse_molecule_file,
            render_gaussian,
            validate_chemical_spec
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
