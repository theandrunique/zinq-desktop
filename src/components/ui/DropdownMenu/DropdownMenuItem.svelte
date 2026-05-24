<script lang="ts">
  import { DropdownMenu } from "bits-ui";
  import { cn } from "@/utils";

  let { children, class: className, ...rest }: DropdownMenu.ItemProps = $props();

  let itemClasses = $derived(
    cn(
      "relative flex cursor-pointer select-none items-center gap-2 rounded-[var(--radius)] px-1 py-1 text-sm text-[var(--color-text)] outline-none hover:bg-[var(--color-interactive-hover)] focus:bg-[var(--color-interactive-hover)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
      className,
    ),
  );
</script>

<DropdownMenu.Item class={itemClasses} {...rest}>
  {@render children?.()}
</DropdownMenu.Item>
