import { create } from "zustand";
import { type AppState, type Command } from "../bindings";
import { commands } from "../bindings";

type AppStore = {
  state: AppState | null;
  loadError: string | null;
  loadInitialState: () => Promise<void>;
  dispatchCommand: (command: Command) => Promise<void>;
  applyCommands: (commands: Command[]) => Promise<void>;
  undo: () => Promise<void>;
  redo: () => Promise<void>;
};

// Rust commands use serde's camelCase field names at the Tauri boundary.
// The generated TypeScript types currently expose the Rust field names, so
// normalize command payload keys in one place before invoking Rust.
function toTauriCommand(command: Command): Command {
  return Object.fromEntries(
    Object.entries(command).map(([key, value]) => [
      key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase()),
      value,
    ]),
  ) as Command;
}

export const useAppStore = create<AppStore>((set) => ({
  state: null,
  loadError: null,
  loadInitialState: async () => {
    try {
      const state = await commands.getStateTauri();
      set({ state, loadError: null });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ loadError: message });
    }
  },
  dispatchCommand: async (command) => {
    const nextState = await commands.applyCommandTauri(toTauriCommand(command));
    set({ state: nextState });
  },
  applyCommands: async (cmds) => {
    let nextState = null;
    for (const command of cmds) {
      nextState = await commands.applyCommandTauri(toTauriCommand(command));
    }
    if (nextState) {
      set({ state: nextState });
    }
  },
  undo: async () => {
    const nextState = await commands.undoTauri();
    set({ state: nextState });
  },
  redo: async () => {
    const nextState = await commands.redoTauri();
    set({ state: nextState });
  },
}));
