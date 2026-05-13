import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { type AppState } from "../domain/chemicalSpec";
import type { Command } from "../domain/commands";

type AppStore = {
  state: AppState | null;
  past: AppState[];
  loadInitialState: () => Promise<void>;
  dispatchCommand: (command: Command) => Promise<void>;
  applyCommands: (commands: Command[]) => Promise<void>;
  undo: () => void;
  canUndo: () => boolean;
};

export const useAppStore = create<AppStore>((set, get) => ({
  state: null,
  past: [],
  loadInitialState: async () => {
    if (get().state) return;
    const state = await invoke<AppState>("get_initial_app_state");
    set({ state, past: [] });
  },
  dispatchCommand: async (command) => {
    const current = get().state;
    if (!current) return;
    const next = await invoke<AppState>("apply_command", { state: current, command });
    set(({ past }) => ({
      state: next,
      past: [...past, current].slice(-30),
    }));
  },
  applyCommands: async (commands) => {
    if (commands.length === 0) return;
    const current = get().state;
    if (!current) return;
    const next = await invoke<AppState>("apply_commands", { state: current, commands });
    set(({ past }) => ({
      state: next,
      past: [...past, current].slice(-30),
    }));
  },
  undo: () =>
    set(({ state, past }) => {
      const previous = past[past.length - 1];
      if (!previous) return { state, past };
      return {
        state: previous,
        past: past.slice(0, -1),
      };
    }),
  canUndo: () => get().past.length > 0,
}));
