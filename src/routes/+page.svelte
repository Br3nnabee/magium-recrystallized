<script lang="ts">
  import { writable } from "svelte/store";
  import { base } from "$app/paths";

  // Scroll store
  const scrollY = writable(0);
  function handleScroll() {
    scrollY.set(window.scrollY);
  }

  // Contributors list
  // TODO: Break out into seperate file or automate pulling
  const contributors = [
    {
      name: "AIR200",
      avatar: "https://avatars.githubusercontent.com/u/122616805?s=60&v=4",
    },
    {
      name: "Ark",
      avatar:
        "https://cdn.discordapp.com/avatars/766538287543615529/01d9fe98428f370daac5d8222fa8daa8.webp",
    },
    {
      name: "aviewer.",
      avatar:
        "https://cdn.discordapp.com/avatars/508345520318840833/e9447291b48f38792191867f249d5eaf.webp",
    },
    {
      name: "Br3nnabee",
      avatar: "https://avatars.githubusercontent.com/u/142729024?v=4",
    },
    {
      name: "Cheese Guy",
      avatar:
        "https://cdn.discordapp.com/avatars/923801513640493117/400d704d3c2e23b55ee323e015f0f65e.webp",
    },
    {
      name: "Colaboi2009",
      avatar: "https://avatars.githubusercontent.com/u/133582178?s=60&v=4",
    },
    {
      name: "DrongoDyle",
      avatar:
        "https://cdn.discordapp.com/avatars/398733259464245249/07526682c7e54cd22a8ff8ded4d7e7c3.webp",
    },
    {
      name: "Evan",
      avatar:
        "https://cdn.discordapp.com/avatars/517013816135909377/40e8f4a10d7b542bddee7e66f6222016.webp",
    },
    {
      name: "Hasten",
      avatar:
        "https://cdn.discordapp.com/avatars/211244797904683018/eac4f6119cccc6333bf1731dc3d0568f.webp",
    },
    {
      name: "himanshunaidu",
      avatar: "https://avatars.githubusercontent.com/u/24753890?s=60&v=4",
    },
    {
      name: "Ipcdragon",
      avatar: "https://avatars.githubusercontent.com/u/179416299?s=60&v=4",
    },
    {
      name: "Lord Lucas",
      avatar:
        "https://cdn.discordapp.com/avatars/997523720631504936/6c0e1657353dd69c0d54a9be12d8c56c.webp",
    },
    {
      name: "Moops",
      avatar:
        "https://cdn.discordapp.com/avatars/312570716619014144/aaea7b756d9c6136d918a3985918d7aa.webp",
    },
    {
      name: "Rin Est",
      avatar:
        "https://cdn.discordapp.com/guilds/1278729249947386047/users/286149710031486976/avatars/a2b5f898c4ddd10a3c2dda34e0b05ea1.webp",
    },
    {
      name: "Treatised War",
      avatar:
        "https://cdn.discordapp.com/avatars/953566479926841406/a1413016d7c92ee02ac7276243273a89.webp",
    },
    {
      name: "thuiop",
      avatar: "https://avatars.githubusercontent.com/u/1338337?s=60&v=4",
    },
    {
      name: "WhateverComic07",
      avatar:
        "https://cdn.discordapp.com/avatars/206523849414541314/252c124215ec75412202af3fe8f1129d.webp",
    },
    {
      name: "ZiClaud",
      avatar: "https://avatars.githubusercontent.com/u/56027915?s=60&v=4",
    },
  ];
</script>

<svelte:window on:scroll={handleScroll} />

<svelte:head>
  <title>Magium: Recrystallized</title>
  <meta
    name="description"
    content="Dreams, duels, and death. One mortal's path to magic."
  />
</svelte:head>

<main
  class="min-h-screen text-[#FAF5E4] bg-gradient-to-br from-[#3F1D1D] via-[#1C0A0A] to-[#120404]"
>
  <!-- Hero Section -->
  <section
    class="relative h-screen flex flex-col items-center justify-center text-center px-6"
    style:opacity={$scrollY < 50 ? 1 : Math.max(0, 1 - ($scrollY - 50) / 500)}
  >
    <div
      class="absolute inset-0 flex items-center justify-center pointer-events-none"
    >
      <svg
        width="90%"
        height="90%"
        viewBox="0 0 800 600"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <!-- TODO: Change to scale percentage to retain text inside in mobile -->
        <rect
          x="10"
          y="10"
          width="780"
          height="580"
          stroke="#D69E2E"
          stroke-width="8"
          rx="40"
        />
        <circle cx="50" cy="50" r="20" fill="#D69E2E" />
        <circle cx="750" cy="550" r="20" fill="#D69E2E" />
      </svg>
      <img
        src="{base}/logo-red.png"
        alt="Game Logo"
        class="absolute aspect-square w-[max(30vw,75vh)] opacity-30 drop-shadow-2xl"
      />
    </div>

    <h1
      class="relative z-10 text-5xl md:text-6xl font-black tracking-wide drop-shadow-lg"
    >
      Magium: Recrystallized
    </h1>
    <p class="mt-4 text-lg md:text-xl max-w-xl leading-relaxed drop-shadow-md">
      Dreams, duels, and death. One mortal's path to magic.
    </p>
    <div
      class="flex flex-col sm:flex-row items-center justify-center space-y-4 sm:space-y-0 sm:space-x-8 mt-8 z-10"
    >
      <a
        href="{base}/play"
        class="inline-block px-8 py-3 font-semibold bg-[#D69E2E] text-[#3F1D1D]
        rounded-full border-2 border-[#FAF5E4] hover:bg-[#b9891f] transition-all shadow-inner"
      >
        Play in Browser
      </a>
      <a
        href=""
        class="inline-block px-8 py-3 font-semibold text-[#D69E2E] rounded-full
        border-2 border-[#D69E2E] hover:bg-[#D69E2E] hover:text-[#3F1D1D] transition-colors"
      >
        Download App
      </a>
    </div>
  </section>

  <!-- About Section -->
  <section class="py-20 bg-[#2E1111]">
    <div class="max-w-3xl mx-auto px-6 text-center space-y-6">
      <h2 class="text-3xl md:text-4xl font-bold text-[#D69E2E]">
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
  </section>

  <!-- Join the Effort Section -->
  <section class="py-20 bg-[#3F1D1D]">
    <div class="max-w-6xl mx-auto px-6">
      <h2 class="text-3xl md:text-4xl font-bold text-[#D69E2E] text-center">
        Join the Effort
      </h2>
      <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-8">
        <div
          class="p-8 bg-[#3F1D1D] rounded-2xl border-2 border-[#D69E2E] hover:bg-[#472322] transition-colors"
        >
          <h3 class="text-2xl font-bold mb-4">Write & Curate</h3>
          <p class="mb-6">
            Contribute to the story and wiki: Refine lore and write the very
            story of Magium.
          </p>
          <div class="flex flex-col space-y-4">
            <a
              href="https://discord.gg/VrTHsqKd93"
              target="_blank"
              class="px-6 py-3 bg-[#D69E2E] text-[#3F1D1D] font-semibold rounded-full
              border-2 border-[#FAF5E4] hover:bg-[#b9891f] transition-all"
            >
              Join the Writers' Discord
            </a>
            <a
              href="https://magium.fandom.com/wiki/Magium_Wiki"
              target="_blank"
              class="px-6 py-3 text-[#D69E2E] font-semibold rounded-full border-2
              border-[#D69E2E] hover:bg-[#D69E2E] hover:text-[#3F1D1D] transition-colors"
            >
              Contribute to the Wiki
            </a>
          </div>
        </div>
        <div
          class="p-8 bg-[#3F1D1D] rounded-2xl border-2 border-[#D69E2E] hover:bg-[#472322] transition-colors"
        >
          <h3 class="text-2xl font-bold mb-4">Code & Improve</h3>
          <p class="mb-6">
            Dive into the codebase: fix bugs, add features, and optimize
            performance.
          </p>
          <div class="flex flex-col space-y-4">
            <a
              href=""
              target="_blank"
              class="px-6 py-3 bg-[#D69E2E] text-[#3F1D1D] font-semibold rounded-full
              border-2 border-[#FAF5E4] hover:bg-[#b9891f] transition-all"
            >
              Contributing Guide
            </a>
            <a
              href="https://github.com/Br3nnabee/magium-recrystallized"
              target="_blank"
              class="px-6 py-3 text-[#D69E2E] font-semibold rounded-full border-2
              border-[#D69E2E] hover:bg-[#D69E2E] hover:text-[#3F1D1D] transition-colors"
            >
              GitHub Repo
            </a>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- Contributors Display -->
  <section class="py-10 bg-gradient-to-t from-[#120404] to-[#2E1111]">
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
