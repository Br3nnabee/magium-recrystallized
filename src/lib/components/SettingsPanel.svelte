<script lang="ts">
  import { onMount } from "svelte";
  import {
    textWidthStore,
    colorThemeStore,
    TextWidth,
    ColorTheme,
    useMaxWidth,
  } from "$lib/stores/displaysettings";

  let isDark = false;

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

  function onSetColor(theme: ColorTheme) {
    colorThemeStore.set(theme);
  }

  function onSetWidth(width: TextWidth) {
    textWidthStore.set(width);
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
             rounded-lg shadow hover:shadow-md transition"
    >
      {#if isDark}
        Disable Dark Mode
      {:else}
        Enable Dark Mode
      {/if}
    </button>

    <div class="flex flex-row space-x-7">
      {#each Object.values(ColorTheme) as theme}
        <button
          on:click={() => onSetColor(theme)}
          class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium
                 bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900
                 rounded-lg shadow hover:shadow-md transition
                 {$colorThemeStore === theme
            ? 'ring-2 ring-offset-2 ring-indigo-500'
            : ''}"
        >
          {theme.charAt(0).toUpperCase() + theme.slice(1)}
        </button>
      {/each}
    </div>

    {#if $useMaxWidth}
      <div class="flex flex-row space-x-7">
        {#each [{ label: "Full Width", value: TextWidth.Full }, { label: "3/4 Width", value: TextWidth.Medium }, { label: "1/2 Width", value: TextWidth.Low }] as { label, value }}
          <button
            on:click={() => onSetWidth(value)}
            class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium
                   bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900
                   rounded-lg shadow hover:shadow-md transition
                   {$textWidthStore === value
              ? 'ring-2 ring-offset-2 ring-indigo-500'
              : ''}"
          >
            {label}
          </button>
        {/each}
      </div>
    {/if}
  </nav>
</div>
