<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "@/utils";

  let { class: className, ...rest }: Dialog.OverlayProps = $props();

  let cls = $derived(
    cn(
      "dialog-overlay data-nested:hidden fixed inset-0 z-50 bg-black/60 backdrop-blur-xs backdrop-blur-[2px]",
      className,
    ),
  );
</script>

<Dialog.Overlay class={cls} {...rest} />

<style>
  @keyframes enter {
    from {
      opacity: 0;
    }
  }
  @keyframes exit {
    to {
      opacity: 0;
    }
  }

  :global(.dialog-overlay[data-state="open"]) {
    animation: enter 150ms ease-out;
  }

  :global(.dialog-overlay[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
