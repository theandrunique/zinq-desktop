<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { cn } from "@/utils";

  type Props = HTMLInputAttributes & {
    value?: string;
  };

  let { value = $bindable(""), class: className = "", ...rest }: Props = $props();

  let classes = $derived(
    cn(
      "p-1 px-2 border border-[var(--color-border)] rounded-[var(--radius)] text-sm bg-[var(--color-bg-input)] hover:bg-[var(--color-bg-input-hover)] text-[var(--color-text)] placeholder:text-[var(--color-text-muted)] placeholder:font-semibold",
      "focus:outline-[2px] focus:outline-[var(--color-focus)] focus:outline-offset-0 focus:border-[var(--color-focus)]",
      "disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-[var(--gray-3)]",
      className,
    ),
  );
</script>

<input bind:value class={classes} {...rest} />
