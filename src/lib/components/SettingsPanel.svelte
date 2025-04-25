<script lang="ts">
  import { onMount } from "svelte";

  let isDark = false;

  onMount(() => {
    const stored = localStorage.getItem("theme");

    if (
      stored === "dark" ||
      (stored === null &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
      isDark = true;
    }

    document.documentElement.classList.toggle("dark", isDark);

    localStorage.setItem("theme", isDark ? "dark" : "light");
  });

  function onToggle() {
    document.documentElement.classList.toggle("dark", isDark);
    localStorage.setItem("theme", isDark ? "dark" : "light");
  }
</script>

<div class="p-8">
  <h2
    class=" text-4xl font-extrabold text-neutral-900 dark:text-neutral-50 mb-8"
  >
    Settings
  </h2>

  <div
    class="flex items-center justify-between
           w-full min-w-[12rem]
           px-6 py-4
           text-lg font-medium
           bg-gray-800 text-neutral-50
           dark:bg-gray-50 dark:text-neutral-900
           rounded-lg shadow
           hover:shadow-md transition
           "
  >
    <span> Dark Mode </span>

    <label class="relative inline-flex items-center cursor-pointer">
      <!-- hidden checkbox -->
      <input
        type="checkbox"
        class="sr-only peer"
        bind:checked={isDark}
        on:change={onToggle}
      />

      <!-- track -->
      <div
        class="w-12 h-6 rounded-full
           bg-gray-700 dark:bg-gray-300
           peer-focus:ring-4 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800
           transition-colors duration-200
           peer-checked:bg-primary-600"
      ></div>

      <!-- thumb -->
      <div
        class="absolute left-0.5 top-0.5 w-5 h-5 rounded-full
           bg-neutral-800 dark:bg-neutral-100
           transition-transform duration-200
           peer-checked:translate-x-6"
      ></div>
    </label>
  </div>
</div>
