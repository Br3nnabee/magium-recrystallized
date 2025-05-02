<!--
  Svelte component responsible for rendering and laying out
  a dynamic list of choice buttons based on the current story node.
  It measures button sizes, computes an optimal column grid,
  and reacts to window resizing and story state changes.
-->

<script lang="ts" context="module">
  /**
   * A helper type mapping a choice label to its async action.
   */
  export interface Choice {
    /** Text displayed on the choice button */
    label: string;
    /** Async callback invoked when the choice is selected */
    action: () => Promise<void>;
  }
</script>

<script lang="ts">
  import { onMount, tick, onDestroy } from "svelte";
  import { initialize, currentNode, goTo } from "$lib/stores/passagestore";
  import ChoiceButton from "./ChoiceButton.svelte";

  // Reactive list of mapped choices for the current node
  let choices: Choice[] = [];
  // References to container elements for layout computation
  let container: HTMLDivElement;
  let measureContainer: HTMLDivElement;

  // Layout state: number of columns and maximum button width
  let columns = 1;
  let maxButtonWidth = 0;
  // Gap (in pixels) between buttons in the grid
  const GAP = 16;

  /**
   * Initialize the story on mount and register resize listener.
   */
  onMount(() => {
    // Load initial node and set up store subscriptions
    initialize();
    // Recompute layout when window size changes
    window.addEventListener("resize", onResize);
  });

  /**
   * Clean up the window resize listener on destroy.
   */
  onDestroy(() => {
    window.removeEventListener("resize", onResize);
  });

  /**
   * Handler invoked on window resize events.
   * Updates layout based on the new container width.
   */
  function onResize() {
    if (container) updateLayout(container.clientWidth);
  }

  /**
   * Reactive block: rebuild the Choice[] array whenever the current node changes.
   */
  $: {
    const node = $currentNode;
    // Map raw edges into Choice actions that call goTo
    choices = node.edges.map((e) => ({
      label: e.label,
      action: async () => {
        if (e.dest >= 0) {
          await goTo(e.dest);
        }
      },
    }));
  }

  /**
   * Reactive block: when choices are present, measure their rendered widths.
   */
  $: if (choices.length) measureSizes();

  /**
   * Measure each button's width in a hidden container to determine max width,
   * then update the grid layout accordingly.
   */
  async function measureSizes() {
    // Wait for DOM update
    await tick();
    if (!measureContainer) return;

    // Compute maximum width among all choice buttons
    maxButtonWidth = Math.max(
      ...Array.from(measureContainer.children).map(
        (c) => (c as HTMLElement).getBoundingClientRect().width,
      ),
    );

    // Trigger layout recalculation with the new width
    if (container) updateLayout(container.clientWidth);
  }

  /**
   * Compute the optimal number of columns for the given container width.
   * It tries to balance rows and columns to form a square-like grid,
   * while respecting the maximum button width and gap constraints.
   *
   * @param width - Available pixel width of the container element
   */
  function updateLayout(width: number) {
    const n = choices.length;
    // Maximum columns that fit based on button width + gap
    const maxCols = Math.max(
      1,
      Math.floor((width + GAP) / (maxButtonWidth + GAP)),
    );

    // If all buttons fit in fewer columns, use that count directly
    if (n <= maxCols) {
      columns = n;
      return;
    }

    // Generate factor pairs (rows, cols) for n choices
    const pairs: { rows: number; cols: number }[] = [];
    for (let i = 1; i * i <= n; i++) {
      if (n % i === 0) {
        pairs.push({ rows: n / i, cols: i }, { rows: i, cols: n / i });
      }
    }

    // Pick the pair with smallest row-col difference that fits maxCols
    const best = pairs
      .filter((p) => p.cols <= maxCols)
      .sort((a, b) => {
        const da = Math.abs(a.rows - a.cols);
        const db = Math.abs(b.rows - b.cols);
        // Prefer more square (minimize difference), then more columns
        return da !== db ? da - db : b.cols - a.cols;
      })[0];

    // Fallback to at least one column if no pair fits
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
