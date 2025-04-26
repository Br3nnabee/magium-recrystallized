<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { get } from "svelte/store";
  import {
    uiState,
    PrimaryState,
    MenuSubstate,
    closeMenu,
    openMenu,
    toggleMenu,
  } from "$lib/stores/state";
  import AchievementsPanel from "./AchievementsPanel.svelte";
  import SavesPanel from "./SavesPanel.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";
  import AboutPanel from "./AboutPanel.svelte";

  const THRESHOLD = 28 * 16 * 2.2;
  let innerWidth = typeof window !== "undefined" ? window.innerWidth : 0;
  $: useMaxWidth = innerWidth > THRESHOLD;
  let menuEl: HTMLElement;
  let menuWidth = 0;

  function checkWidth() {
    innerWidth = window.innerWidth;
    useMaxWidth = innerWidth > THRESHOLD;
  }

  async function measureMenu() {
    await tick();
    if (menuEl) menuWidth = menuEl.getBoundingClientRect().width;
  }

  $: document.body.style.overflow =
    $uiState.primary === PrimaryState.Menu ? "hidden" : "";

  function onKeyup(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopImmediatePropagation();
      const state = get(uiState);
      toggleMenu();
    }
  }

  onMount(() => {
    checkWidth();
    window.addEventListener("resize", checkWidth, { passive: true });
    window.addEventListener("resize", measureMenu, { passive: true });
    window.addEventListener("keyup", onKeyup);
  });

  onDestroy(() => {
    window.removeEventListener("resize", checkWidth);
    window.removeEventListener("resize", measureMenu);
    window.removeEventListener("keyup", onKeyup);
  });

  function scrollLock(node: HTMLElement, locked: boolean) {
    const set = (val: boolean) =>
      (document.body.style.overflow = val ? "hidden" : "");
    set(locked);
    return {
      update(v: boolean) {
        set(v);
      },
      destroy() {
        document.body.style.overflow = "";
      },
    };
  }

  function trapFocus(node: HTMLElement) {
    const prev = document.activeElement as HTMLElement | null;
    node.focus({ preventScroll: true });
    const sel =
      'a[href],button:not([disabled]),textarea,input,select,[tabindex]:not([tabindex="-1"])';
    let focusables: HTMLElement[] = [];
    const refresh = () =>
      (focusables = Array.from(node.querySelectorAll<HTMLElement>(sel)));
    refresh();
    function onKey(e: KeyboardEvent) {
      if (e.key !== "Tab") return;
      refresh();
      const idx = focusables.indexOf(document.activeElement as HTMLElement);
      if (e.shiftKey) {
        if (idx === 0 || document.activeElement === node) {
          e.preventDefault();
          focusables[focusables.length - 1]?.focus();
        }
      } else {
        if (idx === focusables.length - 1 || document.activeElement === node) {
          e.preventDefault();
          focusables[0]?.focus();
        }
      }
    }
    node.addEventListener("keydown", onKey);
    return {
      destroy() {
        node.removeEventListener("keydown", onKey);
        prev?.focus();
      },
    };
  }

  $: if ($uiState.primary === PrimaryState.Menu) measureMenu();

  function quicksave() {
    console.log("Quicksave");
  }
  function loadLastCheckpoint() {
    console.log("Load last checkpoint");
  }

  interface MenuButton {
    label: string;
    action: () => void;
  }
  const buttons: MenuButton[] = [
    { label: "Quicksave", action: quicksave },
    { label: "Last Checkpoint", action: loadLastCheckpoint },
    {
      label: "Achievements",
      action: () => openMenu(MenuSubstate.Achievements),
    },
    { label: "Saves", action: () => openMenu(MenuSubstate.Saves) },
    { label: "Settings", action: () => openMenu(MenuSubstate.Settings) },
    { label: "About", action: () => openMenu(MenuSubstate.About) },
  ];
</script>

{#if $uiState.primary === PrimaryState.Menu}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-[rgba(10,10,10,0.6)] backdrop-blur-sm z-40"
    on:click={closeMenu}
    aria-hidden="true"
    transition:fade
  />

  <!-- Main Menu -->
  <div
    bind:this={menuEl}
    use:scrollLock={$uiState.primary === PrimaryState.Menu}
    use:trapFocus
    tabindex="-1"
    class="fixed inset-y-0 left-0 z-50 p-8 overflow-auto w-full h-[100dvh] bg-gray-50 dark:bg-gray-900"
    class:max-w-md={useMaxWidth}
    class:max-w-full={!useMaxWidth}
    role="dialog"
    aria-modal="true"
    on:click|stopPropagation
    in:fly={{ x: -innerWidth, duration: 300 }}
    out:fly={{ x: -innerWidth, duration: 300 }}
  >
    <!-- Close -->
    <button
      class="absolute top-6 p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-800 focus:outline-none focus:ring"
      class:left-6={useMaxWidth}
      class:right-6={!useMaxWidth}
      aria-label="Close menu"
      on:click={closeMenu}
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

    <nav class="mt-16 space-y-6">
      <h2
        class="text-4xl font-extrabold text-neutral-900 dark:text-neutral-50"
        class:text-left={useMaxWidth}
        class:text-center={!useMaxWidth}
      >
        Menu
      </h2>
      <ul class="space-y-3">
        {#each buttons as { label, action }}
          <li>
            <button
              class="w-full min-w-[12rem] px-6 py-4 text-lg font-medium bg-gray-800 text-neutral-50
              dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition"
              on:click={action}
              class:text-left={useMaxWidth}
              class:text-center={!useMaxWidth}
            >
              {label}
            </button>
          </li>
        {/each}
      </ul>
    </nav>
  </div>
{/if}

{#if $uiState.primary === PrimaryState.Menu && $uiState.substate != null}
  <!-- Substate Overlay -->
  <div
    class="fixed top-0 bottom-0 bg-gray-200 dark:bg-gray-800 z-50 overflow-auto"
    style={useMaxWidth ? `left: ${menuWidth}px; right: 0;` : ""}
    class:inset-0={!useMaxWidth}
    role="region"
    aria-label={$uiState.substate}
    on:click|stopPropagation
    in:fly={{
      x: useMaxWidth ? innerWidth - menuWidth : -innerWidth,
      duration: 300,
    }}
    out:fly={{
      x: useMaxWidth ? innerWidth - menuWidth : -innerWidth,
      duration: 300,
    }}
  >
    {#if !useMaxWidth}
      <button
        class="absolute top-6 right-6 p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring"
        aria-label="Close subpanel"
        on:click={() => openMenu()}
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
    {/if}

    {#if $uiState.substate === MenuSubstate.Achievements}
      <AchievementsPanel />
    {:else if $uiState.substate === MenuSubstate.Saves}
      <SavesPanel />
    {:else if $uiState.substate === MenuSubstate.Settings}
      <SettingsPanel {useMaxWidth} />
    {:else if $uiState.substate === MenuSubstate.About}
      <AboutPanel />
    {/if}
  </div>
{/if}
