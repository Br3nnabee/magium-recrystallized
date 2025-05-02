/*
 * Manages user interface preferences: text width, color theme, and responsive max-width flag.
 * Uses Svelte stores backed by localStorage and window events.
 */

import { readable, writable } from 'svelte/store';

/** Available text width options */
export enum TextWidth {
  Full = "full",
  Medium = "medium",
  Low = "low",
}

// Attempt to read saved text width from localStorage
const storedWidth = localStorage.getItem("textWidth") as TextWidth | null;
// Validate saved value or default to Full
const initialWidth: TextWidth =
  storedWidth && Object.values(TextWidth).includes(storedWidth)
    ? storedWidth
    : TextWidth.Full;

/** Writable store for text width preference */
export const textWidthStore = writable<TextWidth>(initialWidth);

// Persist text width changes to localStorage
textWidthStore.subscribe((width) => {
  localStorage.setItem("textWidth", width);
});

/** Available color theme options */
export enum ColorTheme {
  Neutral = "neutral",
  Cool = "cool",
  Warm = "warm",
}

/**
 * Apply a color theme to the document root and save it.
 * @param theme - Selected ColorTheme value
 */
function applyColorTheme(theme: ColorTheme) {
  document.documentElement.dataset.theme = theme;
  localStorage.setItem("colorTheme", theme);
}

// Initialize theme from localStorage or default to Neutral
const storedColor = localStorage.getItem("colorTheme") as ColorTheme;
export const colorThemeStore = writable<ColorTheme>(
  storedColor && Object.values(ColorTheme).includes(storedColor)
    ? storedColor
    : ColorTheme.Neutral
);

// Apply theme on changes
colorThemeStore.subscribe(applyColorTheme);

/**
 * Pixel threshold above which the UI should use its max-width layout.
 * Computed as 28rem (assuming 16px base) * 2.2 scaling factor.
 */
const THRESHOLD = 28 * 16 * 2.2;

/**
 * Readable store that tracks if window width exceeds THRESHOLD.
 * Useful for responsive layout decisions.
 */
export const useMaxWidth = readable<boolean>(false, (set) => {
  function update() {
    set(window.innerWidth > THRESHOLD);
  }

  if (typeof window !== 'undefined') {
    update();
    window.addEventListener('resize', update, { passive: true });
  }

  // Cleanup listener on unsubscribe
  return () => {
    window.removeEventListener('resize', update);
  };
});
