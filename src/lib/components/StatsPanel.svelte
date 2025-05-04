<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fly, fade } from 'svelte/transition';
  import type { StatKey, Stats } from '$lib/stores/stats';
  import { statsStore } from '$lib/stores/stats';
  import { uiState, PrimaryState, openGame } from '$lib/stores/state';

  // Base stat keys
  const statKeys: StatKey[] = [
    'Strength','Speed','Toughness','Reflexes',
    'Hearing','Observation','Ancient Languages',
    'Combat Technique','Premonition','Bluff',
    'Magical sense','Aura hardening'
  ];

  const firstBatch = statKeys.slice(0, 6);
  const restBatch = statKeys.slice(6);

  // Subscribe to combined stats store
  let current: Stats = {} as Stats;
  const unsubscribe = statsStore.subscribe(vals => current = vals);
  onDestroy(unsubscribe);

  // Increment a base stat when clicked
  function handleClick(key: StatKey) {
    statsStore.addStat(key, 1);
  }

  /**
   * Action to toggle scroll locking on <body>
   */
  function scrollLock(node: HTMLElement, locked: boolean) {
    const set = (val: boolean) => (document.body.style.overflow = val ? 'hidden' : '');
    set(locked);
    return {
      update(v: boolean) { set(v); },
      destroy() { document.body.style.overflow = ''; }
    };
  }

  /**
   * Action to trap focus within the stats panel
   */
  function trapFocus(node: HTMLElement) {
    const prev = document.activeElement as HTMLElement | null;
    node.focus({ preventScroll: true });

    const sel = 'a[href],button:not([disabled]),textarea,input,select,[tabindex]:not([tabindex="-1"])';
    let focusables: HTMLElement[] = [];
    const refresh = () => focusables = Array.from(node.querySelectorAll<HTMLElement>(sel));
    refresh();

    function onKey(e: KeyboardEvent) {
      if (e.key !== 'Tab') return;
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
    node.addEventListener('keydown', onKey);

    return {
      destroy() {
        node.removeEventListener('keydown', onKey);
        prev?.focus();
      }
    };
  }

  // Global Escape-to-close handler
  function onKeyup(e: KeyboardEvent) {
    if (e.key === 'Escape' && $uiState.primary === PrimaryState.Stats) {
      openGame();
    }
  }

  // Lock page scroll reactively when stats panel is open
  $: document.body.style.overflow = $uiState.primary === PrimaryState.Stats ? 'hidden' : '';

  onMount(() => {
    window.addEventListener('keyup', onKeyup);
  });

  onDestroy(() => {
    window.removeEventListener('keyup', onKeyup);
  });
</script>

{#if $uiState.primary === PrimaryState.Stats}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-[rgba(10,10,10,0.6)] backdrop-blur-sm z-40"
    on:click={openGame}
    aria-hidden="true"
    transition:fade
  />

  <!-- Stats Panel (dialog) -->
  <div
    use:scrollLock={$uiState.primary === PrimaryState.Stats}
    use:trapFocus
    tabindex="-1"
    role="dialog"
    aria-modal="true"
    class="fixed inset-0 z-50 bg-white dark:bg-gray-900 overflow-auto"
    on:click|stopPropagation
    in:fly={{ x: innerWidth, duration: 300 }}
    out:fly={{ x: innerWidth, duration: 300 }}
  >
    <div class="h-full flex flex-col sm:p-6">
      <!-- Header with close action -->
      <header class="relative mb-4 flex-shrink-0 text-center">
        <button
          class="absolute top-6 right-6 p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-800 focus:outline-none focus:ring"
          aria-label="Close stats panel"
          on:click={() => openGame()}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-neutral-600 dark:text-neutral-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
        <h2 class="text-4xl font-extrabold text-neutral-900 dark:text-neutral-50 mt-[6rem]">Stats</h2>
      </header>

      <!-- Top metrics -->
      <div class="mb-4 flex justify-center space-x-6 flex-shrink-0">
        <div class="py-2">
          <span class="font-medium mr-1">Magical power:</span>
          <span class="font-semibold">{current.magicalPower}</span>
        </div>
        <div class="py-2">
          <span class="font-medium mr-1">Magical knowledge:</span>
          <span class="font-semibold">{current.magicalKnowledge}</span>
        </div>
      </div>

      <!-- Available points -->
      <div class="mb-6 text-center font-medium flex-shrink-0">
        Available points: {current.availablePoints}
      </div>

      <!-- Base stats list -->
      <div class="flex-grow flex flex-col p-4 justify-center space-y-6">
        <ul class="grid grid-cols-2 gap-y-2 gap-x-3">
          {#each firstBatch as key}
            <li>
              <button
                class="w-full flex justify-between items-center px-3 py-2 text-sm sm:text-base font-medium bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition focus:outline-none"
                on:click={() => handleClick(key)}
              >
                <span class="truncate">{key}</span>
                <span class="ml-2 flex-shrink-0">{current[key]}</span>
              </button>
            </li>
          {/each}
        </ul>

        <ul class="grid grid-cols-1 gap-y-2">
          {#each restBatch as key}
            <li>
              <button
                class="w-full flex justify-between items-center px-3 py-2 text-sm sm:text-base font-medium bg-gray-800 text-neutral-50 dark:bg-gray-50 dark:text-neutral-900 rounded-lg shadow hover:shadow-md transition focus:outline-none"
                on:click={() => handleClick(key)}
              >
                <span class="truncate">{key}</span>
                <span class="ml-2 flex-shrink-0">{current[key]}</span>
              </button>
            </li>
          {/each}
        </ul>
      </div>
    </div>
  </div>
{/if}
