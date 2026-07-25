<script lang="ts">
  import { formatDateSeparator } from "./utils";

  interface Props {
    timestamp: string;
  }

  let { timestamp }: Props = $props();

  let label = $derived(formatDateSeparator(timestamp));
</script>

<div class="pointer-events-none sticky top-2 z-10 mb-1 flex w-auto items-center justify-center">
  <div class="rounded-full bg-(--gray-5)/80 px-3 py-1 text-xs font-semibold shadow-lg">
    {label}
  </div>
</div>
