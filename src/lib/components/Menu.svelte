<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { menuOpen } from "$lib/stores/menu";

  const THRESHOLD = 28 * 16 * 3; // 3 × 28 rem

  let useMaxWidth = false;

  const toggle = () => menuOpen.update((v) => !v);

  function onKeyup(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      event.stopImmediatePropagation(); /* block any later esc listeners.
      Not sure why it bugs out otherwise since I'm p sure it's the only esc listener */
      toggle();
    }
  }

  function checkWidth() {
    useMaxWidth = window.innerWidth > THRESHOLD;
  }

  onMount(() => {
    checkWidth();

    window.addEventListener("resize", checkWidth, { passive: true });
    window.addEventListener("keyup", onKeyup);

    return () => {
      window.removeEventListener("resize", checkWidth);
      window.removeEventListener("keyup", onKeyup);
    };
  });
</script>

{#if $menuOpen}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-[rgba(10,10,10,0.6)] backdrop-blur-sm z-40"
    on:click={toggle}
    aria-hidden="true"
    transition:fade
  />

  <!-- Sliding panel -->
  <aside
    class="
      fixed inset-y-0 left-0 z-50
      p-8 overflow-auto w-full
      bg-gray-100 dark:bg-gray-900
    "
    class:max-w-md={useMaxWidth}
    class:max-w-full={!useMaxWidth}
    role="dialog"
    aria-modal="true"
    on:click|stopPropagation
    transition:fly={{ x: -innerWidth, duration: 300 }}
  >
    <!-- Close button -->
    <button
      class="absolute top-6 right-6 p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-800 focus:outline-none focus:ring"
      aria-label="Close menu"
      on:click={toggle}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-8 w-8 text-neutral-600 dark:text-neutral-300"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M6 18L18 6M6 6l12 12"
        />
      </svg>
    </button>

    <!-- Menu content -->
    <nav class="mt-16 space-y-6">
      <h2
        class="text-4xl font-extrabold text-neutral-800 dark:text-neutral-100"
        class:text-left={useMaxWidth}
        class:text-center={!useMaxWidth}
      >
        Menu
      </h2>
      <ul class="space-y-3">
        {#each ["Quicksave", "Last Checkpoint", "Achievements", "Saves", "Settings", "About"] as item}
          <li>
            <button
              class="w-full min-w-[12rem] px-6 py-4 text-lg font-medium bg-gray-800
                     text-neutral-100 dark:bg-gray-100 dark:text-neutral-800
                     rounded-lg shadow hover:shadow-md transition"
              on:click={() => console.log(item)}
              class:text-left={useMaxWidth}
              class:text-center={!useMaxWidth}
            >
              {item}
            </button>
          </li>
        {/each}
      </ul>
    </nav>
  </aside>
{/if}
