<script lang="ts">
  import { DropdownMenu } from "bits-ui";
  import { cn } from "@/utils";

  let {
    children,
    class: className,
    sideOffset = 4,
    ...props
  }: DropdownMenu.ContentProps = $props();

  let contentClasses = $derived(
    cn(
      "dropdown-menu-content z-50 max-h-[--bits-dropdown-menu-content-available-height] min-w-[8rem] overflow-y-hidden rounded-md p-1",
      "bg-[var(--color-bg)] shadow-[0_8px_30px_rgb(0,0,0,0.8)]",
      "origin-[--bits-dropdown-menu-content-transform-origin]",
      className,
    ),
  );
</script>

<DropdownMenu.Content class={contentClasses} {sideOffset} {...props}>
  {@render children?.()}
</DropdownMenu.Content>

<style>
  @keyframes enter {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
  }
  @keyframes exit {
    to {
      opacity: 0;
      transform: scale(0.95);
    }
  }

  :global(.dropdown-menu-content[data-state="open"]) {
    animation: enter 150ms ease-out;
  }

  :global(.dropdown-menu-content[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
