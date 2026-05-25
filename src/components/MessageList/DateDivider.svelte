<script lang="ts">
  import { formatDateSeparator } from "./utils";

  interface Props {
    timestamp: string;
  }

  let { timestamp }: Props = $props();

  let label = $derived(formatDateSeparator(timestamp));
</script>

<div class="pointer-events-none flex w-auto items-center justify-center">
  <div
    class="mb-2 rounded-full bg-(--gray-3)/80 px-3 py-1 text-xs font-semibold text-white shadow-lg"
  >
    {label}
  </div>
</div>
