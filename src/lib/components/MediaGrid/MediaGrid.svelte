<script lang="ts">
  export type ImageMeta = {
    src: string;
    width: number;
    height: number;
  };

  let {
    images = [],
    onclick,
    class: className,
    rowHeight = 150,
    maxHeight,
    gap = 2,
    maxItemsPerRow = 4,
  }: {
    images?: ImageMeta[];
    onclick?: (item: ImageMeta, index: number) => void;
    class?: string;
    rowHeight?: number;
    maxHeight?: number;
    gap?: number;
    maxItemsPerRow?: number;
  } = $props();

  type RowDef = { items: ImageMeta[]; startIdx: number };

  let containerWidth = $state(0);

  function* genPartitions(n: number): Generator<number[]> {
    if (n === 0) {
      yield [];
      return;
    }
    for (let first = 1; first <= Math.min(n, maxItemsPerRow); first++) {
      for (const rest of genPartitions(n - first)) {
        yield [first, ...rest];
      }
    }
  }

  function findBestPattern(ars: number[]): number[] {
    const n = ars.length;
    if (n <= 1) return [1];
    if (n === 2) {
      if (ars[0] > 1 && ars[1] > 1) return [1, 1];
      return [2];
    }
    if (n === 3) return [2, 1];

    let best: number[] = [];
    let bestCost = Infinity;

    for (const p of genPartitions(n)) {
      let cost = 0;
      let idx = 0;
      for (const cols of p) {
        let sumAr = 0;
        for (let j = 0; j < cols; j++) sumAr += ars[idx + j];
        idx += cols;
        let availableWidth = containerWidth - (cols - 1) * gap;
        let h = availableWidth / sumAr;
        if (maxHeight && h > maxHeight) h = maxHeight;
        cost += Math.abs(h - rowHeight);
        if (cols > 2) cost += (cols - 2) * rowHeight * 0.3;
      }
      if (cost < bestCost) {
        bestCost = cost;
        best = p;
      }
    }
    return best;
  }

  function buildRows(): RowDef[] {
    if (images.length === 0 || containerWidth === 0) return [];
    const ars = images.map((im) => im.width / im.height);
    const pattern = findBestPattern(ars);
    const result: RowDef[] = [];
    let idx = 0;
    for (const cols of pattern) {
      result.push({ items: images.slice(idx, idx + cols), startIdx: idx });
      idx += cols;
    }
    return result;
  }

  let rows: RowDef[] = $derived(buildRows());

  function handleClick(item: ImageMeta, index: number) {
    onclick?.(item, index);
  }
</script>

<div bind:clientWidth={containerWidth} class="flex flex-col {className ?? ''}" style="gap: {gap}px">
  {#each rows as row (row.startIdx)}
    {#if row.items.length === 1}
      {@const im = row.items[0]}
      {@const ar = im.width / im.height}
      <div class="flex" style="gap: {gap}px">
        <button
          type="button"
          class="overflow-hidden border-none bg-none p-0"
          style="width: 100%; aspect-ratio: {ar}; max-height: {maxHeight
            ? `${maxHeight}px`
            : 'none'}"
          onclick={onclick ? () => handleClick(im, row.startIdx) : undefined}
        >
          <img
            src={im.src}
            alt=""
            class="block h-full w-full object-cover"
            draggable="false"
            loading="lazy"
          />
        </button>
      </div>
    {:else}
      <div class="flex" style="gap: {gap}px">
        {#each row.items as im, j (im.src)}
          {@const ar = im.width / im.height}
          {@const globalIdx = row.startIdx + j}
          <button
            type="button"
            class="overflow-hidden border-none bg-none p-0"
            style="flex: {ar}; min-width: 0; aspect-ratio: {ar}; max-height: {maxHeight
              ? `${maxHeight}px`
              : 'none'}"
            onclick={onclick ? () => handleClick(im, globalIdx) : undefined}
          >
            <img
              src={im.src}
              alt=""
              class="block h-full w-full object-cover"
              draggable="false"
              loading="lazy"
            />
          </button>
        {/each}
      </div>
    {/if}
  {/each}
</div>
