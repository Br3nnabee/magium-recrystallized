<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import ChapterContent from "$lib/components/ChapterContent.svelte";
  import ChoiceList from "$lib/components/ChoiceList.svelte";
  import Menu from "$lib/components/Menu.svelte";

  const book = 1;
  const chapter = 1;

  const choices = [
    { label: "Explore the forest", action: () => console.log("Forest") },
    { label: "Enter the cave", action: () => console.log("Cave") },
    { label: "Return to town", action: () => console.log("Town") },
  ];
  document.documentElement.setAttribute("data-theme", "neutral");
</script>

<div
  class="min-h-screen flex flex-col bg-gray-100 dark:bg-gray-900 text-neutral-900 dark:text-neutral-100"
>
  <Header {book} {chapter} />
  <Menu />
  <main class="flex-1 overflow-auto">
    <div class="w-full px-6 py-10 prose lg:prose-lg lg:px-16">
      <ChapterContent />
      <div class="mt-8">
        <ChoiceList {choices} />
      </div>
    </div>
  </main>
</div>
