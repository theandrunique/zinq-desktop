<script lang="ts">
  import type { Attachment } from "@/types";
  import MediaItem from "./MediaItem.svelte";

  interface Props {
    attachments: Attachment[];
  }

  let { attachments }: Props = $props();

  let total = $derived(attachments.length);
</script>

{#if total === 1}
  <MediaItem attachment={attachments[0]} class="h-full max-h-[250px] w-full" />
{:else if total === 2}
  <div class="grid w-full grid-cols-2 gap-1">
    {#each attachments as attachment (attachment.id)}
      <MediaItem {attachment} class="aspect-[4/3] w-full" />
    {/each}
  </div>
{:else if total === 3}
  <div class="grid max-h-[450px] w-full grid-rows-2 gap-1">
    <MediaItem attachment={attachments[0]} class="max-h-[250px] w-full" />
    <div class="grid h-full grid-cols-2 gap-1">
      {#each attachments.slice(1) as attachment (attachment.id)}
        <MediaItem {attachment} class="aspect-[4/3] w-full" />
      {/each}
    </div>
  </div>
{:else if total === 4}
  <div class="grid max-h-[450px] w-full grid-rows-2 gap-1">
    <MediaItem attachment={attachments[0]} class="row-span-1 w-full" />
    <div class="grid h-full grid-cols-3 gap-1">
      {#each attachments.slice(1) as attachment (attachment.id)}
        <MediaItem {attachment} class="aspect-[4/3] w-full" />
      {/each}
    </div>
  </div>
{:else}
  <div class="grid h-full w-full grid-rows-2 gap-1">
    <MediaItem attachment={attachments[0]} class="h-full row-span-1 w-full" />
    <div class="relative grid h-full grid-cols-3 gap-1">
      {#each attachments.slice(1, 5) as attachment (attachment.id)}
        <MediaItem {attachment} class="w-full" />
      {/each}
      {#if total > 5}
        <div
          class="absolute right-0 bottom-0 flex h-full w-full items-center justify-center rounded-lg bg-black/60 text-lg font-bold text-white"
        >
          +{total - 5}
        </div>
      {/if}
    </div>
  </div>
{/if}
