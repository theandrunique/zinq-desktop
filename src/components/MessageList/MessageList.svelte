<script lang="ts">
  import type { Message } from "@/types";
  import { groupMessagesByAuthor, groupMessagesByDate } from "./utils";
  import DateDivider from "./DateDivider.svelte";
  import MessageGroup from "./MessageGroup.svelte";
  import ScrollButton from "./ScrollButton.svelte";
  import { createScrollManager } from "./scroll-manager.svelte";

  interface Props {
    messages: Message[];
    currentUserId: string;
    maxReadMessageId?: string;
    onLoadAroundMessage?: (targetId: string) => Promise<void>;
    onLoadOlder?: () => void;
  }

  let {
    messages,
    currentUserId,
    maxReadMessageId = "",
    onLoadAroundMessage,
    onLoadOlder,
  }: Props = $props();

  let dateGroups = $derived(groupMessagesByDate(messages));
  let containerEl = $state<HTMLElement | null>(null);

  let manager = $state(createScrollManager());
  let initialScrolled = $state(false);
  let prevMessagesLength = $state(0);

  $effect(() => {
    if (containerEl) manager.setContainer(containerEl);
  });

  $effect(() => {
    if (maxReadMessageId) manager.setLastReadId(maxReadMessageId);
  });

  $effect(() => {
    return () => manager.destroy();
  });

  $effect(() => {
    if (initialScrolled || !containerEl) return;
    if (messages.length === 0) return;

    const raf = requestAnimationFrame(() => {
      if (maxReadMessageId) {
        manager.scrollToMessage(maxReadMessageId, { behavior: "instant" });
      }
      manager.updateButtonState();
      initialScrolled = true;
    });

    return () => cancelAnimationFrame(raf);
  });

  $effect(() => {
    const len = messages.length;
    if (prevMessagesLength > 0 && len > prevMessagesLength && containerEl) {
      manager.scrollToBottomOnNewMessage();
    }
    prevMessagesLength = len;
  });

  $effect(() => {
    const el = containerEl;
    if (!el) return;

    function onScroll() {
      if (!el) return;
      if (el.scrollTop < 200) {
        onLoadOlder?.();
      }
    }

    el.addEventListener("scroll", onScroll);
    return () => el.removeEventListener("scroll", onScroll);
  });

  async function handleNavigateToMessage(fromId: string, toId: string) {
    if (manager.hasElement(toId)) {
      manager.navigateToReference(fromId, toId);
      return;
    }
    if (onLoadAroundMessage) {
      await onLoadAroundMessage(toId);
      manager.navigateToReference(fromId, toId);
    }
  }
</script>

<div class="relative h-full">
  <div bind:this={containerEl} class="h-full overflow-y-auto" style="overflow-anchor: auto">
    <div class="my-2 flex flex-col gap-2 px-4">
      {#each dateGroups as [dateKey, dateMessages] (dateKey)}
        <div class="flex flex-col gap-2">
          <DateDivider timestamp={dateMessages[0].created_at} />
          {#each groupMessagesByAuthor(dateMessages) as group (group[0].id)}
            <MessageGroup
              {group}
              {currentUserId}
              {maxReadMessageId}
              messageAction={manager.messageAction}
              onNavigateToMessage={handleNavigateToMessage}
            />
          {/each}
        </div>
      {/each}
    </div>
  </div>
  <ScrollButton {manager} />
</div>
