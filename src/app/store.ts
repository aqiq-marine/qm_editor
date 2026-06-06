import { create } from "zustand";
import { type AppState } from "../domain/chemicalSpec";
import type { Command } from "../domain/commands";
import { commands } from "../bindings";

type AppStore = {
  state: AppState | null;
  loadInitialState: () => Promise<void>;
  dispatchCommand: (command: Command) => Promise<void>;
  applyCommands: (commands: Command[]) => Promise<void>;
  undo: () => Promise<void>;
  redo: () => Promise<void>;
};

export const useAppStore = create<AppStore>((set) => ({
  state: null,
  loadInitialState: async () => {
    const state = await commands.getStateTauri();
    set({ state });
  },
  dispatchCommand: async (command) => {
    const nextState = await commands.applyCommandTauri(command);
    set({ state: nextState });
  },
  applyCommands: async (cmds) => {
    let nextState = null;
    for (const command of cmds) {
      nextState = await commands.applyCommandTauri(command);
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
