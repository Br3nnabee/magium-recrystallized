<script lang="ts">
  import { onMount } from "svelte";
  import { textWidth } from "$lib/stores/displaysettings";

  export let useMaxWidth: boolean;

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

  function setWidth() {}
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
      class="w-full min-w-[12rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
    dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
    >
      {#if isDark}
        Disable Dark Mode
      {:else}
        Enable Dark Mode
      {/if}
    </button>
    {#if useMaxWidth === true}
      <div class="flex flex-row space-x-7">
        <button
          on:click={setWidth}
          class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
        >
          Full Width
        </button>
        <button
          on:click={setWidth}
          class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
        >
          3/4 Width
        </button>
        <button
          on:click={setWidth}
          class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
        >
          1/2 Width
        </button>
      </div>
    {/if}
    <div class="flex flex-row space-x-7">
      <button
        on:click={setWidth}
        class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
      >
        Neutral
      </button>
      <button
        on:click={setWidth}
        class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
      >
        Cool
      </button>
      <button
        on:click={setWidth}
        class="w-full min-w-[5rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
        dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
      >
        Warm
      </button>
    </div>
  </nav>
</div>
