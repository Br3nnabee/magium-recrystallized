<script lang="ts" context="module">
  /**
   * A helper type for mapping edge label → action
   */
  export interface Choice {
    label: string;
    action: () => Promise<void>;
  }
</script>

<script lang="ts">
  import { onMount, tick, onDestroy } from "svelte";
  import { initialize, currentNode, goTo } from "$lib/stores/passagestore";
  import ChoiceButton from "./ChoiceButton.svelte";

  // Reactive state
  let choices: Choice[] = [];
  let container: HTMLDivElement;
  let measureContainer: HTMLDivElement;
  let columns = 1;
  let maxButtonWidth = 0;
  const GAP = 16;

  // On mount: initialize story and listen for resize
  onMount(() => {
    initialize();
    window.addEventListener("resize", onResize);
  });

  // Cleanup listener
  onDestroy(() => {
    window.removeEventListener("resize", onResize);
  });

  function onResize() {
    if (container) updateLayout(container.clientWidth);
  }

  // Rebuild choices whenever currentNode changes
  $: {
    const node = $currentNode;
    choices = node.edges.map((e) => ({
      label: e.label,
      action: async () => {
        if (e.dest >= 0) {
          await goTo(e.dest);
        }
      },
    }));
  }

  // Measure button widths to compute grid layout
  $: if (choices.length) measureSizes();

  async function measureSizes() {
    await tick();
    if (!measureContainer) return;
    maxButtonWidth = Math.max(
      ...Array.from(measureContainer.children).map(
        (c) => (c as HTMLElement).getBoundingClientRect().width,
      ),
    );
    if (container) updateLayout(container.clientWidth);
  }

  function updateLayout(width: number) {
    const n = choices.length;
    const maxCols = Math.max(
      1,
      Math.floor((width + GAP) / (maxButtonWidth + GAP)),
    );
    if (n <= maxCols) {
      columns = n;
      return;
    }
    const pairs: { rows: number; cols: number }[] = [];
    for (let i = 1; i * i <= n; i++) {
      if (n % i === 0) {
        pairs.push({ rows: n / i, cols: i }, { rows: i, cols: n / i });
      }
    }
    const best = pairs
      .filter((p) => p.cols <= maxCols)
      .sort((a, b) => {
        const da = Math.abs(a.rows - a.cols);
        const db = Math.abs(b.rows - b.cols);
        return da !== db ? da - db : b.cols - a.cols;
      })[0];
    columns = best?.cols || 1;
  }
</script>

<!-- Hidden measurer for button widths -->
<div
  bind:this={measureContainer}
  style="position:absolute; top:-9999px; left:-9999px; visibility:hidden;"
>
  {#each choices as c (c.label)}
    <ChoiceButton label={c.label} action={c.action} />
  {/each}
</div>

<!-- Actual choice grid -->
<div
  bind:this={container}
  class="grid gap-4"
  style="grid-template-columns: repeat({columns}, minmax(0, 1fr));"
>
  {#each choices as c (c.label)}
    <ChoiceButton label={c.label} action={c.action} />
  {/each}
</div>
