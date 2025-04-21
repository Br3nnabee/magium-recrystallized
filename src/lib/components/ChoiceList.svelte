<script context="module" lang="ts">
  export interface Choice {
    label: string;
    action: () => void;
  }
</script>

<script lang="ts">
  import ChoiceButton from "./ChoiceButton.svelte";
  export let choices: Choice[] = [];
</script>

<div class="flex flex-wrap justify-center gap-4">
  {#each choices as choice (choice.label)}
    <ChoiceButton label={choice.label} action={choice.action} />
  {/each}
</div>
