<script lang="ts">
  import { Avatar } from "@/components/ui";
  import { renderMessageText } from "@/utils";
  import type { Message } from "@/types";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();

  let text = $derived(renderMessageText(message));
  let imageUrl = $derived.by(() => {
    if (message.type === "CHAT_IMAGE_UPDATE") return message.metadata.new_image;
    return null;
  });
</script>

<div
  data-message-id={message.id}
  data-message-author-id={message.author_id}
  class="mb-2 text-center text-sm text-(--color-text)"
>
  {#if imageUrl}
    <div class="flex flex-col items-center gap-2">
      <div class="opacity-50">{text}</div>
      <Avatar src={imageUrl} alt="Group image" fallback="" class="h-24 w-24" />
    </div>
  {:else}
    <div class="opacity-50">{text}</div>
  {/if}
</div>
