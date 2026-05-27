<script lang="ts">
  import { Avatar } from "@/components/ui";
  import type { Message } from "@/types";
  import { isMetaMessage } from "./utils";
  import MessageBubble from "./MessageBubble.svelte";
  import MetaMessage from "./content/MetaMessage.svelte";

  interface Props {
    group: Message[];
    currentUserId: string;
    maxReadMessageId?: string;
    messageAction: (node: HTMLElement, id: string) => { destroy: () => void };
    onNavigateToMessage?: (fromId: string, toId: string) => void;
  }

  let {
    group,
    currentUserId,
    maxReadMessageId = "",
    messageAction,
    onNavigateToMessage,
  }: Props = $props();

  let firstMessage = $derived(group[0]);
  let isOwn = $derived(firstMessage.author_id === currentUserId);
  let isMeta = $derived(isMetaMessage(firstMessage));
  let avatarUser = $derived(firstMessage.author);
</script>

{#if isMeta}
  {#each group as message (message.id)}
    <div use:messageAction={message.id}>
      <MetaMessage {message} />
    </div>
  {/each}
{:else}
  <div class="flex w-full gap-2">
    {#if !isOwn}
      <div class="sticky bottom-2 shrink-0 self-end text-[0]">
        <Avatar
          src={avatarUser.avatar}
          alt={avatarUser.global_name}
          fallback={avatarUser.global_name.charAt(0).toUpperCase()}
          class="h-8 w-8"
        />
      </div>
    {/if}
    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
      {#each group as message, i (message.id)}
        <MessageBubble
          {message}
          isOwnMessage={isOwn}
          isFirstInGroup={i === 0}
          isLastInGroup={i === group.length - 1}
          {maxReadMessageId}
          {messageAction}
          {onNavigateToMessage}
        />
      {/each}
    </div>
  </div>
{/if}
