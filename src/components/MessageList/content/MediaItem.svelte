<script lang="ts">
  import type { Attachment } from "@/types";

  interface Props {
    attachment: Attachment;
    class?: string;
  }

  let { attachment, class: className = "" }: Props = $props();

  let isImage = $derived(attachment.content_type.startsWith("image/"));
  let isVideo = $derived(attachment.content_type.startsWith("video/"));
</script>

{#if isImage}
  <img
    src={attachment.storage_key}
    alt={attachment.filename}
    class="{className} rounded-lg object-cover"
  />
{:else if isVideo}
  <div class="{className} overflow-hidden rounded-lg">
    <video controls src={attachment.storage_key} class="h-full w-full object-cover">
      <track kind="captions" />
    </video>
  </div>
{/if}
