<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "@/utils";
  import { X } from "@lucide/svelte";

  let { class: className, ...rest }: Dialog.CloseProps = $props();

  let cls = $derived(
    cn(
      "absolute top-2 right-2 bg-transparent hover:bg-[var(--color-interactive-hover)] p-1 rounded-[var(--radius)] cursor-pointer",
      className,
    ),
  );
</script>

<Dialog.Close class={cls} {...rest}>
  <X class="h-4 w-4" />
</Dialog.Close>
