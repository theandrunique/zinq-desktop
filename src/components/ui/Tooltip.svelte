<script lang="ts">
  import { Tooltip } from "bits-ui";
  import type { Snippet } from "svelte";
  import { cn } from "@/utils";

  type Props = Tooltip.RootProps & {
    trigger: Snippet;
    content: Snippet;
    triggerProps?: Tooltip.TriggerProps;
  };

  let { trigger, content, triggerProps, ...rest }: Props = $props();

  let contentClasses = $derived(
    cn(
      "tooltip z-50 overflow-hidden rounded-md px-1.5 py-1 text-xs font-bold shadow-lg",
      "bg-[var(--color-bg-tooltip)] text-[var(--color-text-tooltip)]",
    ),
  );
</script>

<Tooltip.Root {...rest}>
  <Tooltip.Trigger {...triggerProps}>
    {@render trigger()}
  </Tooltip.Trigger>
  <Tooltip.Portal>
    <Tooltip.Content side="top" class={contentClasses} sideOffset={8} collisionPadding={8}>
      {@render content()}
    </Tooltip.Content>
  </Tooltip.Portal>
</Tooltip.Root>

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

  :global(.tooltip) {
    animation: enter 150ms ease-out;
  }

  :global(.tooltip[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
