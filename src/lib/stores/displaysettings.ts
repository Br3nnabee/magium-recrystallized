import { readable, writable } from "svelte/store";

export enum TextWidth {
  Full = "full",
  Medium = "medium",
  Low = "low",
}

const storedWidth = localStorage.getItem("textWidth") as TextWidth | null;
const initialWidth: TextWidth =
  storedWidth && Object.values(TextWidth).includes(storedWidth)
    ? storedWidth
    : TextWidth.Full;

export const textWidthStore = writable<TextWidth>(initialWidth);

textWidthStore.subscribe((width) => {
  localStorage.setItem("textWidth", width);
});

export enum ColorTheme {
  Neutral = "neutral",
  Cool = "cool",
  Warm = "warm",
}

function applyColorTheme(theme: ColorTheme) {
  document.documentElement.dataset.theme = theme;
  localStorage.setItem("colorTheme", theme);
}

const storedColor = localStorage.getItem("colorTheme") as ColorTheme;
export const colorThemeStore = writable<ColorTheme>(
  storedColor && Object.values(ColorTheme).includes(storedColor)
    ? storedColor
    : ColorTheme.Neutral
);

colorThemeStore.subscribe(applyColorTheme);


const THRESHOLD = 28 * 16 * 2.2;

export const useMaxWidth = readable<boolean>(false, (set) => {
  function update() {
    set(window.innerWidth > THRESHOLD);
  }

  if (typeof window !== 'undefined') {
    update();
    window.addEventListener('resize', update, { passive: true });
  }

  return () => {
    window.removeEventListener('resize', update);
  };
});
