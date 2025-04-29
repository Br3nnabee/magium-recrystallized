<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import {
    textWidthStore,
    colorThemeStore,
    TextWidth,
    ColorTheme,
    useMaxWidth,
  } from "$lib/stores/displaysettings";

  let isDark = false;
  let themeDirection = 1;
  let widthDirection = 1;

  const themes = Object.values(ColorTheme);
  $: themeIndex = themes.findIndex((t) => t === $colorThemeStore);

  const widths = [
    { label: "Full Width", value: TextWidth.Full },
    { label: "3/4 Width", value: TextWidth.Medium },
    { label: "1/2 Width", value: TextWidth.Low },
  ];
  $: widthIndex = widths.findIndex((w) => w.value === $textWidthStore);

  onMount(() => {
    const stored = localStorage.getItem("theme");
    isDark =
      stored === "dark" ||
      (stored === null &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);
    applyTheme();
  });

  function applyTheme() {
    document.documentElement.classList.toggle("dark", isDark);
    localStorage.setItem("theme", isDark ? "dark" : "light");
  }

  function onToggleDark() {
    isDark = !isDark;
    applyTheme();
  }

  function prevTheme() {
    themeDirection = -1;
    const next = (themeIndex - 1 + themes.length) % themes.length;
    colorThemeStore.set(themes[next]);
  }

  function nextTheme() {
    themeDirection = 1;
    const next = (themeIndex + 1) % themes.length;
    colorThemeStore.set(themes[next]);
  }

  function prevWidth() {
    widthDirection = -1;
    const next = (widthIndex - 1 + widths.length) % widths.length;
    textWidthStore.set(widths[next].value);
  }

  function nextWidth() {
    widthDirection = 1;
    const next = (widthIndex + 1) % widths.length;
    textWidthStore.set(widths[next].value);
  }

  // Click division handlers
  function onThemeClick(e: MouseEvent) {
    const btn = e.currentTarget as HTMLElement;
    e.offsetX < btn.clientWidth / 2 ? prevTheme() : nextTheme();
  }

  function onWidthClick(e: MouseEvent) {
    const btn = e.currentTarget as HTMLElement;
    e.offsetX < btn.clientWidth / 2 ? prevWidth() : nextWidth();
  }
</script>

<div class="p-8">
  <nav class="mt-16 space-y-6">
    <h2
      class="text-4xl font-extrabold text-neutral-900 dark:text-neutral-50 mb-8"
    >
      Settings
    </h2>

    <button
      on:click={onToggleDark}
      class="w-full min-w-[12rem] px-6 py-4 text-lg font-medium
             bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900
             rounded-lg shadow hover:shadow-md transition-colors duration-200"
    >
      {#if isDark}
        Disable Dark Mode
      {:else}
        Enable Dark Mode
      {/if}
    </button>

    <button
      on:click={onThemeClick}
      class="w-full min-w-[12rem] flex flex-nowrap items-center justify-between
             px-6 py-4 text-lg font-medium
             bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900
             rounded-lg shadow hover:shadow-md transition-colors duration-200"
      aria-label="Cycle color themes"
    >
      <span class="text-xl">◀</span>

      <div
        class="flex-1 text-center overflow-hidden whitespace-nowrap"
        style="min-width: 7rem;"
      >
        {#key themes[themeIndex]}
          <span
            class="inline-block w-full capitalize"
            in:fly={{ x: themeDirection * 50, duration: 200 }}
            out:fly={{ x: -themeDirection * 50, duration: 200 }}
          >
            {themes[themeIndex]}
          </span>
        {/key}
      </div>

      <span class="text-xl">▶</span>
    </button>

    {#if $useMaxWidth}
      <button
        on:click={onWidthClick}
        class="w-full min-w-[12rem] flex flex-nowrap items-center justify-between
               px-6 py-4 text-lg font-medium
               bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900
               rounded-lg shadow hover:shadow-md transition-colors duration-200"
        aria-label="Cycle text widths"
      >
        <span class="text-xl">◀</span>

        <div
          class="flex-1 text-center overflow-hidden whitespace-nowrap"
          style="min-width: 7rem;"
        >
          {#key widths[widthIndex].value}
            <span
              class="inline-block w-full capitalize"
              in:fly={{ x: widthDirection * 50, duration: 200 }}
              out:fly={{ x: -widthDirection * 50, duration: 200 }}
            >
              {widths[widthIndex].label}
            </span>
          {/key}
        </div>

        <span class="text-xl">▶</span>
      </button>
    {/if}
  </nav>
</div>
