<script lang="ts">
  import type { Message } from "@/types";
  import MediaGrid from "./MediaGrid.svelte";
  import FileCard from "./FileCard.svelte";

  interface Props {
    message: Message;
    isOwnMessage: boolean;
  }

  let { message, isOwnMessage: _isOwnMessage = false }: Props = $props();

  let imagesAndVideos = $derived(
    message.attachments.filter(
      (a) => a.content_type.startsWith("image/") || a.content_type.startsWith("video/"),
    ),
  );
  let audios = $derived(message.attachments.filter((a) => a.content_type.startsWith("audio/")));
  let others = $derived(
    message.attachments.filter(
      (a) =>
        !a.content_type.startsWith("image/") &&
        !a.content_type.startsWith("video/") &&
        !a.content_type.startsWith("audio/"),
    ),
  );
</script>

{#if message.attachments.length > 0}
  <div class="space-y-2">
    {#if imagesAndVideos.length > 0}
      <MediaGrid attachments={imagesAndVideos} />
    {/if}
    {#each audios as audio (audio.id)}
      <FileCard attachment={audio} isAudio />
    {/each}
    {#each others as file (file.id)}
      <FileCard attachment={file} />
    {/each}
  </div>
{/if}
