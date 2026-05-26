<script lang="ts">
  import { Check, CheckCheck } from "@lucide/svelte";
  import type { Message } from "@/types";
  import { formatMessageTime } from "./utils";
  import { cn } from "@/utils";

  interface Props {
    message: Message;
    isOwnMessage: boolean;
    maxReadMessageId?: string;
    class?: string;
  }

  let { message, isOwnMessage, maxReadMessageId = "", class: className = "" }: Props = $props();

  let isEdited = $derived(message.edited_at !== null);
  function extractNum(id: string): number {
    return parseInt(id.replace(/\D/g, ""), 10) || 0;
  }
  let isRead = $derived(
    !!maxReadMessageId && extractNum(maxReadMessageId) >= extractNum(message.id),
  );
</script>

<div
  class={cn(
    "flex h-5 items-center justify-end text-[12px] whitespace-nowrap text-(--color-text) opacity-70",
    className,
  )}
>
  <span class="mr-1">
    {#if isEdited}edited
    {/if}
    {formatMessageTime(message.created_at)}
  </span>
  {#if isOwnMessage}
    {#if isRead}
      <CheckCheck class="h-4 w-4 text-fuchsia-100" />
    {:else}
      <Check class="h-4 w-4 text-fuchsia-100" />
    {/if}
  {/if}
</div>
