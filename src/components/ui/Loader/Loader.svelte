<script lang="ts">
  import { cn } from "@/utils";

  let {
    class: className,
    size = "md",
  }: { class?: string; size?: "sm" | "md" | "lg" } = $props();

  const sizes = {
    sm: "h-4 w-4",
    md: "h-8 w-8",
    lg: "h-12 w-12",
  };
</script>

<div
  class={cn(
    "animate-spin rounded-full border-[5px] border-(--color-border) border-t-(--color-accent)",
    sizes[size],
    className,
  )}
></div>
