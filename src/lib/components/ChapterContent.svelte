<script lang="ts">
  import SvelteMarkdown from "@humanspeak/svelte-markdown";
  import { currentNode } from "$lib/stores/passagestore";
</script>

<SvelteMarkdown source={$currentNode.content} />
