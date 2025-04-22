<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import ChapterContent from "$lib/components/ChapterContent.svelte";
  import ChoiceList from "$lib/components/ChoiceList.svelte";
  import Menu from "$lib/components/Menu.svelte";

  const book = 1;
  const chapter = 1;

  document.documentElement.setAttribute("data-theme", "neutral");
</script>

<div
  class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900 text-neutral-900 dark:text-neutral-100"
>
  <header class="fixed w-full top-0 z-50 bg-gray-50 dark:bg-gray-900">
    <Header {book} {chapter} />
    <Menu />
  </header>

  <!-- push the main content down by the header’s height -->
  <main class="flex-1 overflow-auto pt-18">
    <div class="w-full px-6 text-lg space-y-4 lg:px-16">
      <ChapterContent />
      <div class="mt-12 mb-4">
        <ChoiceList />
      </div>
    </div>
  </main>
</div>
