use crate::domain::{AppState, Command};
use crate::reducer::{initial_app_state, reduce};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct StateManager {
    present: AppState,
    past: Vec<AppState>,
    future: Vec<AppState>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            present: initial_app_state(),
            past: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Undo => {
                self.undo();
            }
            Command::Redo => {
                self.redo();
            }
            _ => {
                let previous_state = self.present.clone();
                self.present = reduce(previous_state.clone(), command);
                self.past.push(previous_state);
                self.future.clear();
            }
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(previous) = self.past.pop() {
            self.future.push(self.present.clone());
            self.present = previous;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.future.pop() {
            self.past.push(self.present.clone());
            self.present = next;
            true
        } else {
            false
        }
    }

    pub fn get_state(&self) -> &AppState {
        &self.present
    }
}
