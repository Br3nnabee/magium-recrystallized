<script lang="ts">
  import Menu from "./Menu.svelte";
  import { openMenu, openStats } from "$lib/stores/state";
  import StatsPanel from './StatsPanel.svelte';

  export let book: number;
  export let chapter: number;
</script>

<header
  class="flex items-center bg-gray-900 dark:bg-gray-800 text-neutral-50 px-4 sm:px-6 md:px-8 lg:px-12 py-2"
>
  <button
    class="font-semibold hover:text-neutral-300 text-xl py-2"
    on:click={() => openMenu()}
  >
    MENU
  </button>

  <div class="flex-1 text-center leading-none">
    <h2 class="text-xs -mt-0.5 font-semibold truncate leading-none">
      BOOK&nbsp;{book}
    </h2>

    <h1 class="-mt-0.5 text-xl font-bold truncate leading-[1.05]">
      CHAPTER&nbsp;{chapter}
    </h1>
  </div>

  <button 
    class="font-semibold hover:text-neutral-300 text-xl py-2"
    on:click={() => openStats()} 
  >
    STATS
  </button>
</header>

<Menu />
<StatsPanel />
