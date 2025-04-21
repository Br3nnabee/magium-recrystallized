<script lang="ts">
  import { onMount } from "svelte";
  import { content, loadContent } from "$lib/stores/passagestore";

  onMount(() => {
    loadContent();
  });
</script>

<slot>
  {#each $content as paragraph}
    <p class="text-lg mb-8">{paragraph}</p>
  {/each}
</slot>
