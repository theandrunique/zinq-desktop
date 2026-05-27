<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { cn } from "@/utils";
  import type { ScrollManager } from "./scroll-manager.svelte";

  interface Props {
    manager: ScrollManager;
  }

  let { manager }: Props = $props();

  let buttonState = $derived(manager.buttonState);

  let label = $derived.by(() => {
    if (buttonState === "toBottom") return "Scroll to bottom";
    if (buttonState === "toLastRead") return "Jump to new messages";
    if (buttonState === "goBack") return "Go back";
    return "";
  });
</script>

{#if buttonState !== "hidden"}
  <button
    onclick={() => {
      if (buttonState === "toBottom") manager.scrollToBottom();
      else if (buttonState === "toLastRead") manager.scrollToLastRead();
      else if (buttonState === "goBack") manager.goBack();
    }}
    class={cn(
      "absolute right-4 bottom-4 z-20 flex h-10 w-10 items-center justify-center rounded-full shadow-lg transition-all duration-200 active:scale-95",
      "bg-[var(--gray-4)] enabled:hover:bg-[var(--gray-5)] enabled:active:bg-[var(--gray-6)]",
    )}
    aria-label={label}
    title={label}
  >
    <ChevronDown class={cn("h-5 w-5", buttonState === "goBack" && "rotate-180")} />
  </button>
{/if}
