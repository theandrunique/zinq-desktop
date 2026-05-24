<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "@/utils";

  let { children, class: className, ...rest }: Dialog.ContentProps = $props();

  let cls = $derived(
    cn(
      "dialog-content fixed left-[50%] top-[50%] z-50 translate-x-[-50%]",
      "w-full sm:max-w-[390px] max-w-[calc(100%-2rem)] p-3 rounded-xl bg-(--color-bg)",
      "translate-y-[calc(-50%+var(--bits-dialog-nested-count)*-1.5rem)] scale-[calc(1-var(--bits-dialog-nested-count)*0.05)] transition-all duration-150",
      "text-(--color-text)",
      className,
    ),
  );
</script>

<Dialog.Content
  class={cls}
  {...rest}
  style="transform: scale(calc(1 - var(--bits-dialog-nested-count) * 0.05));
         filter: blur(calc(var(--bits-dialog-nested-count) * 2px));"
>
  {@render children?.()}
</Dialog.Content>

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

  :global(.dialog-content[data-state="open"]) {
    animation: enter 150ms ease-out;
  }

  :global(.dialog-content[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
