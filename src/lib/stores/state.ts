import { writable } from 'svelte/store';

// Primary modes of the app
export enum PrimaryState {
  Game = 'Game',
  Menu = 'Menu',
  Stats = 'Stats',
}

// Optional substates when in Menu
export enum MenuSubstate {
  Achievements = 'Achievements',
  Saves = 'Saves',
  Settings = 'Settings',
  About = 'About',
}


// A union type that covers all valid combinations
export type UIState =
  | { primary: PrimaryState.Game }
  | { primary: PrimaryState.Stats }
  | { primary: PrimaryState.Menu; substate?: MenuSubstate }

export const uiState = writable<UIState>({
  primary: PrimaryState.Game,
});

export function openGame() {
  uiState.set({ primary: PrimaryState.Game });
}

export function openStats() {
  uiState.set({ primary: PrimaryState.Stats });
}

export function openMenu(substate?: MenuSubstate) {
  uiState.set({ primary: PrimaryState.Menu, substate });
}

export function closeMenu() {
  uiState.set({ primary: PrimaryState.Game });
}

export function toggleMenu() {
  uiState.update((s) =>
    s.primary === PrimaryState.Menu
      ? { primary: PrimaryState.Game }
      : { primary: PrimaryState.Menu }
  );
}
