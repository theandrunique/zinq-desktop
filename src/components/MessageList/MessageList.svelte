<script lang="ts">
  import type { Message } from "@/types";
  import { groupMessagesByAuthor, groupMessagesByDate } from "./utils";
  import DateDivider from "./DateDivider.svelte";
  import MessageGroup from "./MessageGroup.svelte";

  interface Props {
    messages: Message[];
    currentUserId: string;
    maxReadMessageId?: string;
  }

  let { messages, currentUserId, maxReadMessageId = "" }: Props = $props();

  let dateGroups = $derived(groupMessagesByDate(messages));
</script>

<div class="my-2 flex flex-col gap-2 px-4">
  {#each dateGroups as [dateKey, dateMessages] (dateKey)}
    <div class="flex flex-col gap-2">
      <DateDivider timestamp={dateMessages[0].created_at} />
      {#each groupMessagesByAuthor(dateMessages) as group (group[0].id)}
        <MessageGroup {group} {currentUserId} {maxReadMessageId} />
      {/each}
    </div>
  {/each}
</div>
