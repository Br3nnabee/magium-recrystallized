<script lang="ts">
  export let label: string;
  export let action: () => void;
</script>

<button
  on:click={action}
  class="px-8 py-4 bg-gray-800 text-lg text-neutral-100 dark:bg-gray-50 dark:text-neutral-800
  rounded-sm font-medium hover:bg-gray-700 dark:hover:bg-gray-200"
>
  {label}
</button>
