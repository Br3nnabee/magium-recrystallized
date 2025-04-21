<script lang="ts">
  import { content } from "$lib/stores/passagestore";
</script>

<slot>
  {#each $content as paragraph}
    <p class="text-lg mb-8">{paragraph}</p>
  {/each}
</slot>
