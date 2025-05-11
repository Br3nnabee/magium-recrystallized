<script lang="ts">
  import { writable } from "svelte/store";
  import { base } from "$app/paths";
  import data from "$lib/contrib.json"

  // Scroll store
  const scrollY = writable(0);
  function handleScroll() {
    scrollY.set(window.scrollY);
  }

  // Window size
  let window_h0 = $state(0)
  let window_w0 = $state(0)
  let window_w = $derived(Math.min(window_h0*1.5, window_w0))
  let window_h = $derived(Math.min(window_w0*1.6, window_h0))
  
  // Contributors list
  const contributors = data;
</script>

<svelte:window on:scroll={handleScroll} bind:innerHeight={window_h0} bind:innerWidth={window_w0}/>

<svelte:head>
  <title >Magium: Recrystallized</title>
  <meta
    name="description"
    content="Dreams, duels, and death. One mortal's path to magic."
  />
</svelte:head>

<main
  class="min-h-screen text-[#FAF5E4] bg-gradient-to-br from-[#00000A] to-[#341638]"
>
  <!-- Hero Section -->
  <section
    class="relative h-screen flex flex-col items-center justify-center text-center px-6"
  >
    <div
      class="absolute inset-0 flex items-center justify-center pointer-events-none"
    >
      <svg
        width="90%"
        height="90%"
        viewBox="0 0 {window_w} {window_h}"
        fill="none"
      >
        <rect x="10" y="10" width="{window_w-20}" height="{window_h-20}" stroke="#D69E2E" stroke-width="8" rx="40" /> 
        <circle cx="50" cy="50" r="20" fill="#D69E2E" />
        <circle cx="{window_w-50}" cy="{window_h-50}" r="20" fill="#D69E2E" />
      </svg>
      <img
        src="{base}/image.png"
        alt="Red Logo"
        class="absolute aspect-square w-[max(30vw,75vh)] opacity-50"
      />
    </div>

    <h1
      class="relative z-10 mx-12 text-3xl sm:text-4xl md:text-6xl font-black tracking-wide drop-shadow-2xl drop-shadow-black"
    >
      Magium: Recrystallized
    </h1>
    <p
      class="mt-4 mx-12 text-sm md:text-xl max-w-xl leading-relaxed drop-shadow-2xl drop-shadow-black"
    >
      Dreams, duels, and death. One mortal's path to magic.
    </p>
    <div
      class="flex flex-col sm:flex-row items-center justify-center space-y-4 sm:space-y-0 sm:space-x-8 mt-8 z-10"
    >
      <a
        href="{base}/play"
        class="w-48 inline-block px-10 py-4 font-semibold bg-[#D69E2E] text-[#3F1D1D]
        rounded-full hover:bg-[#b9891f] transition-all shadow-inner"
      >
        Play Now
      </a>
      <a
        href=""
        class="inline-block px-10 py-4 font-semibold bg-[#D69E2E] text-[#3F1D1D]
        rounded-full hover:bg-[#b9891f] transition-all shadow-inner"
      >
        Download App
      </a>
    </div>
  </section>

  <!-- About Section -->
  <section>
    <div class="max-w-3xl mx-auto px-6 text-center space-y-6 pt-10 pb-14">
      <h2 class="text-center text-3xl font-extrabold text-[#D69E2E]">
        About Magium: Recrystallized
      </h2>
      <blockquote
        class="text-xl italic leading-relaxed max-w-prose mx-auto text-[#FAF5E4]"
      >
        "Magium is a living tapestry of choices. Each decision you make echoes
        through a realm steeped in myth, magic, and untold peril."
      </blockquote>
      <p class="text-lg leading-relaxed max-w-prose mx-auto">
        Magium is a text adventure game similar to old Choose your own adventure
        books (CYOA), in which you play as an ordinary guy named Barry, who
        joins a deadly mage tournament against the most powerful mages in the
        world in the hopes of using the prize to fulfill his lifelong dream of
        becoming a mage himself.
      </p>
      <p class="text-lg leading-relaxed max-w-prose mx-auto">
        Crafted by the late Cristian Mihailescu, whose vision lives on through a
        passionate community dedicated to expanding his legacy. Magium:
        Recrystallized tries its best to continue the story with the knowledge
        we have and share his work with the rest of the world.
      </p>
    </div>
    <hr class="text-[#D69E2E] w-[75vw] m-auto" />
  </section>

  <!-- Join the Effort Section -->
  <section class="py-16 bg-transparent">
    <div class="max-w-6xl mx-auto px-6 grid gap-12">
      <h2 class="text-center text-3xl font-extrabold text-[#D69E2E]">
        Join the Effort
      </h2>

      <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-8">
        <!-- Write & Curate Card -->
        <div
          class="p-8 bg-[#1a092f] border-2 border-[#D69E2E] rounded-2xl shadow-lg transition-colors"
        >
          <h3 class="text-2xl font-bold mb-4 text-[#FAF5E4]">
            Write &amp; Curate
          </h3>
          <p class="mb-6 text-[#d1cfe7]">
            Contribute to the story and wiki: refine lore and shape the future
            of Magium.
          </p>
          <div class="flex flex-col gap-4">
            <a
              href="https://discord.gg/VrTHsqKd93"
              target="_blank"
              class="px-6 py-3 bg-[#D69E2E] text-[#3F1D1D] font-semibold rounded-full
              text-center transition-colors hover:bg-[#b9891f]"
            >
              Join the Writers' Discord
            </a>
            <a
              href="https://magium.fandom.com/wiki/Magium_Wiki"
              target="_blank"
              class="px-6 py-3 font-semibold text-[#D69E2E] border-2 border-[#D69E2E]
              rounded-full text-center transition-colors hover:bg-[#D69E2E] hover:text-[#3F1D1D]"
            >
              Contribute to the Wiki
            </a>
          </div>
        </div>

        <!-- Code & Improve Card -->
        <div
          class="p-8 bg-[#1a092f] border-2 border-[#D69E2E] rounded-2xl shadow-lg transition-colors"
        >
          <h3 class="text-2xl font-bold mb-4 text-[#FAF5E4]">
            Code &amp; Improve
          </h3>
          <p class="mb-6 text-[#d1cfe7]">
            Explore the codebase: fix bugs, add features, and optimize
            performance.
          </p>
          <div class="flex flex-col gap-4">
            <a
              href="https://github.com/Br3nnabee/magium-recrystallized/blob/main/CONTRIBUTING.md"
              target="_blank"
              class="px-6 py-3 bg-[#D69E2E] text-[#3F1D1D] font-semibold rounded-full
              text-center transition-colors hover:bg-[#b9891f]"
            >
              Contributing Guide
            </a>
            <a
              href="https://github.com/Br3nnabee/magium-recrystallized"
              target="_blank"
              class="px-6 py-3 font-semibold text-[#D69E2E] border-2 border-[#D69E2E]
              rounded-full text-center transition-colors hover:bg-[#D69E2E] hover:text-[#3F1D1D]"
            >
              GitHub Repo
            </a>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- Contributors Display -->
  <section class="py-10">
    <div class="max-w-5xl mx-auto px-6">
      <h2 class="text-2xl font-bold text-[#D69E2E] text-center mb-6">
        Contributors
      </h2>
      <div
        class="grid grid-cols-3 sm:grid-cols-5 md:grid-cols-7 lg:grid-cols-9 gap-6"
      >
        {#each contributors as c}
          <div class="text-center">
            <img
              src={c.avatar}
              alt={c.name}
              class="w-20 h-20 rounded-full border-2 border-[#D69E2E] mx-auto"
            />
            <p class="mt-2 text-xs text-[#FAF5E4]">{c.name}</p>
          </div>
        {/each}
      </div>
    </div>
  </section>
</main>

<style>
  /* Empty style section required due to bug - see issue */
</style>
