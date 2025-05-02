/*
 * Manages the UI state of the app using Svelte stores.
 * Provides primary modes (Game, Menu, Stats) and optional menu sub-modes.
 */

import { writable } from 'svelte/store';

/** Main app screens */
export enum PrimaryState {
  Game = 'Game',
  Menu = 'Menu',
  Stats = 'Stats',
}

/** Specific menu tabs when in Menu mode */
export enum MenuSubstate {
  Achievements = 'Achievements',
  Saves = 'Saves',
  Settings = 'Settings',
  About = 'About',
}

/**
 * Full UI state shape:
 * - Always has a primary mode
 * - When primary is Menu, may specify a substate
 */
export type UIState =
  | { primary: PrimaryState.Game }
  | { primary: PrimaryState.Stats }
  | { primary: PrimaryState.Menu; substate?: MenuSubstate };

/** Current UI state store (default: Game) */
export const uiState = writable<UIState>({ primary: PrimaryState.Game });

/** Switch to Game screen */
export function openGame() {
  uiState.set({ primary: PrimaryState.Game });
}

/** Switch to Stats screen */
export function openStats() {
  uiState.set({ primary: PrimaryState.Stats });
}

/**
 * Open Menu screen.
 * @param substate Optional specific menu tab to show
 */
export function openMenu(substate?: MenuSubstate) {
  uiState.set({ primary: PrimaryState.Menu, substate });
}

/** Close Menu and return to Game screen */
export function closeMenu() {
  uiState.set({ primary: PrimaryState.Game });
}

/**
 * Toggle between Menu and Game screens.
 * When opening Menu, no substate is set by default.
 */
export function toggleMenu() {
  uiState.update((s) =>
    s.primary === PrimaryState.Menu
      ? { primary: PrimaryState.Game }
      : { primary: PrimaryState.Menu }
  );
}

