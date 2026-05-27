<script lang="ts">
  import { cn } from "@/utils";
  import type { Message } from "@/types";
  import MessageStatus from "./MessageStatus.svelte";
  import MessageContent from "./MessageContent.svelte";

  interface Props {
    message: Message;
    isOwnMessage: boolean;
    isFirstInGroup: boolean;
    isLastInGroup: boolean;
    maxReadMessageId?: string;
    messageAction: (node: HTMLElement, id: string) => { destroy: () => void };
    onNavigateToMessage?: (fromId: string, toId: string) => void;
  }

  let {
    message,
    isOwnMessage,
    isFirstInGroup,
    isLastInGroup,
    maxReadMessageId = "",
    messageAction,
    onNavigateToMessage,
  }: Props = $props();

  let borderRadius = $derived.by(() => {
    if (isFirstInGroup && isLastInGroup) return "rounded-2xl";
    if (isOwnMessage) {
      return cn("rounded-2xl", {
        "rounded-tr-md": !isFirstInGroup,
        "rounded-br-md": !isLastInGroup,
      });
    }
    return cn("rounded-2xl", {
      "rounded-bl-md": !isLastInGroup,
      "rounded-tl-md": !isFirstInGroup,
    });
  });

  let bubbleBg = $derived(isOwnMessage ? "bg-(--color-accent)" : "bg-(--gray-3)");

  let isReply = $derived(message.type === "REPLY");
  let referencedMessage = $derived.by(() => {
    if (message.type === "REPLY") return message.metadata.referenced_message;
    return null;
  });
</script>

<div
  use:messageAction={message.id}
  data-message-id={message.id}
  data-message-author-id={message.author_id}
  class="flex w-full {isOwnMessage ? 'justify-end' : 'justify-start'}"
>
  <div
    class="relative max-w-[min(90%,28rem)] py-1.5 pr-3 pl-2.5 text-sm text-(--color-text) {borderRadius} {bubbleBg}"
  >
    {#if !isOwnMessage && isFirstInGroup}
      <div class="mb-1 leading-none font-semibold">
        {message.author.global_name}
      </div>
    {/if}

    {#if isReply && referencedMessage}
      <div
        role="button"
        tabindex="0"
        onclick={() => onNavigateToMessage?.(message.id, referencedMessage.id)}
        onkeydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            onNavigateToMessage?.(message.id, referencedMessage.id);
          }
        }}
        class="mt-1 cursor-pointer rounded-md border-l-4 p-2 py-1 leading-none transition-colors hover:brightness-110 {isOwnMessage
          ? 'border-(--purple-7) bg-(--purple-7)/50'
          : 'border-(--purple-8) bg-(--purple-8)/30'}"
      >
        <div class="font-medium text-(--color-text)">{referencedMessage.author.global_name}</div>
        <div class="truncate text-sm text-(--color-text)">
          {#if referencedMessage.attachments.length > 0}
            <span class="opacity-70">
              {referencedMessage.attachments.map((a) => a.filename).join(", ")}
            </span>
            {referencedMessage.content ? ", " : ""}
          {/if}
          {referencedMessage.content}
        </div>
      </div>
    {/if}

    {#if message.attachments.length > 0}
      <MessageContent {message} />
    {/if}

    {#if message.content.length === 0}
      <div class="absolute right-1.5 bottom-0">
        <MessageStatus {message} {isOwnMessage} {maxReadMessageId} />
      </div>
    {:else}
      <div class="wrap-break-words whitespace-pre-wrap">
        {message.content}
        <span class="relative float-right ml-1.5 whitespace-nowrap">
          <MessageStatus {message} {isOwnMessage} {maxReadMessageId} class="opacity-0" />
          <div class="absolute top-1 left-1.5">
            <MessageStatus {message} {isOwnMessage} {maxReadMessageId} />
          </div>
        </span>
      </div>
    {/if}
  </div>
</div>
