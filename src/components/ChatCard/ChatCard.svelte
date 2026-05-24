<script lang="ts">
  import { Check, CheckCheck, Users } from "@lucide/svelte";
  import { cn } from "@/utils";
  import { Avatar } from "@/components/ui";
  import { ChannelType, type Chat } from "@/types";
  import { formatTime, getMessagePreview } from "./utils";

  interface Props {
    chat: Chat;
    currentUserId: string;
    selected?: boolean;
    onclick?: () => void;
  }

  let { chat, currentUserId, selected = false, onclick }: Props = $props();

  let isSavedMessages = $derived(chat.type === ChannelType.DM && chat.members.length === 1);

  let otherMember = $derived(chat.members.find((m) => m.user_id !== currentUserId));

  let displayName = $derived.by(() => {
    if (chat.type === ChannelType.DM) {
      if (!otherMember) return "Saved Messages";
      return `${otherMember.global_name} (${otherMember.username})`;
    }
    return chat.name ?? chat.members.map((m) => m.username).join(", ");
  });

  let avatarSrc = $derived.by(() => {
    if (chat.image) return chat.image;
    if (chat.type === ChannelType.DM && otherMember) {
      return otherMember.avatar;
    }
    return null;
  });

  let avatarFallback = $derived.by(() => {
    if (avatarSrc) return "";
    if (chat.type === ChannelType.DM && otherMember) {
      return otherMember.global_name.charAt(0).toUpperCase();
    }
    const name = chat.name ?? "G";
    return name.charAt(0).toUpperCase();
  });

  let isOwnLastMessage = $derived(chat.last_message?.author_id === currentUserId);

  let isMessageRead = $derived.by(() => {
    if (!chat.last_message) return false;
    return chat.max_read_message_id !== "" && chat.max_read_message_id >= chat.last_message.id;
  });

  let lastMessagePreview = $derived.by(() => {
    return getMessagePreview(chat);
  });

  let timestamp = $derived.by(() => {
    if (!chat.last_message) return "";
    return formatTime(chat.last_message.created_at);
  });
</script>

<button
  class={cn(
    "flex w-full items-center gap-3 px-2 py-2.5 text-left transition-colors duration-100",
    "enabled:hover:bg-(--color-interactive-hover) enabled:active:bg-(--color-interactive-active)",
    selected && "bg-(--color-interactive-active)",
  )}
  {onclick}
>
  <div class="shrink-0">
    {#if isSavedMessages}
      <Avatar src="/bookmark.png" alt="Saved Messages" fallback="" class="h-10 w-10" />
    {:else if avatarSrc}
      <Avatar src={avatarSrc} alt={displayName} fallback="" class="h-10 w-10" />
    {:else}
      <Avatar alt={displayName} fallback={avatarFallback} class="h-10 w-10" />
    {/if}
  </div>

  <div class="min-w-0 flex-1">
    <div class="flex items-center justify-between">
      <div class="flex min-w-0 items-center gap-1 font-semibold">
        {#if chat.type === ChannelType.GROUP_DM}
          <Users class="h-4 w-4 shrink-0 text-(--color-text-muted)" />
        {/if}
        <span class="truncate text-sm text-(--color-text)">
          {displayName}
        </span>
      </div>

      <div class="flex shrink-0 items-center gap-1">
        {#if isOwnLastMessage}
          {#if isMessageRead}
            <CheckCheck class="h-4 w-4 text-(--color-accent)" />
          {:else}
            <Check class="h-4 w-4 text-(--color-accent)" />
          {/if}
        {/if}

        {#if timestamp}
          <span class="text-xs text-(--color-text-muted)">{timestamp}</span>
        {/if}
      </div>
    </div>

    <div class="flex items-center justify-between">
      <div class="truncate text-sm text-(--color-text-muted)">
        {#if lastMessagePreview}
          {lastMessagePreview}
        {:else}
          <span class="italic">No messages yet</span>
        {/if}
      </div>

      {#if chat.unread_count > 0 && !isOwnLastMessage}
        <div
          class="ml-1 flex h-4 w-fit items-center justify-center rounded-full bg-(--color-accent) px-1 text-[10px] font-semibold text-(--color-text)"
        >
          {chat.unread_count}
        </div>
      {/if}
    </div>
  </div>
</button>
