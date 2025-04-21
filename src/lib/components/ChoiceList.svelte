<script context="module" lang="ts">
  export interface Choice {
    label: string;
    action: () => void;
  }
</script>

<script lang="ts">
  import { onMount, tick } from "svelte";
  import ChoiceButton from "./ChoiceButton.svelte";

  // TODO: Optimize this to not have to render hidden

  export let choices: Choice[] = [];

  let container: HTMLDivElement;
  let measureContainer: HTMLDivElement;

  // runtime values
  let columns = 1;
  let maxButtonWidth = 0;

  const GAP = 16;

  // measure the “natural” widths of all buttons
  async function measureSizes() {
    // wait for DOM to update
    await tick();
    if (!measureContainer) return;

    maxButtonWidth = 0;
    for (const child of Array.from(
      measureContainer.children,
    ) as HTMLElement[]) {
      // get the width needed to render label without wrapping
      const w = child.getBoundingClientRect().width;
      if (w > maxButtonWidth) maxButtonWidth = w;
    }

    // recalc layout after measuring
    updateLayout(container.clientWidth);
  }

  function updateLayout(containerWidth: number) {
    const n = choices.length;
    // how many columns could we fit at min width?
    const maxCols = Math.max(
      1,
      Math.floor((containerWidth + GAP) / (maxButtonWidth + GAP)),
    );

    if (n <= maxCols) {
      // everything fits in one row
      columns = n;
      return;
    }

    // find “perfect‐rectangle” factors of n whose cols ≤ maxCols
    type Pair = { rows: number; cols: number };
    const pairs: Pair[] = [];
    for (let i = 1; i * i <= n; i++) {
      if (n % i === 0) {
        const j = n / i;
        if (i <= maxCols) pairs.push({ rows: j, cols: i });
        if (j <= maxCols) pairs.push({ rows: i, cols: j });
      }
    }

    if (pairs.length > 0) {
      pairs.sort((a, b) => {
        const da = Math.abs(a.rows - a.cols);
        const db = Math.abs(b.rows - b.cols);
        if (da !== db) return da - db;
        return b.cols - a.cols;
      });
      columns = pairs[0].cols;
    } else {
      // no exact factorization fits → full‑width stack
      columns = 1;
    }
  }

  onMount(() => {
    measureSizes();

    // re‐measure & re‐layout on container resize
    const ro = new ResizeObserver((entries) => {
      for (const { contentRect } of entries) {
        updateLayout(contentRect.width);
      }
    });
    ro.observe(container);
    return () => ro.disconnect();
  });

  // re‐measure if choices change
  $: if (choices) {
    measureSizes();
  }
</script>

<!-- off‑screen measurement container -->
<div
  bind:this={measureContainer}
  style="position:absolute; top:-9999px; left:-9999px; visibility:hidden;"
>
  {#each choices as choice (choice.label)}
    <ChoiceButton label={choice.label} action={() => {}} />
  {/each}
</div>

<!-- actual grid -->
<div
  bind:this={container}
  class="grid gap-4"
  style="grid-template-columns: repeat({columns}, minmax(0, 1fr));"
>
  {#each choices as choice (choice.label)}
    <ChoiceButton label={choice.label} action={choice.action} />
  {/each}
</div>
