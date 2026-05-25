<script lang="ts">
  import { File, Play } from "@lucide/svelte";
  import type { Attachment } from "@/types";

  interface Props {
    attachment: Attachment;
    isAudio?: boolean;
  }

  let { attachment, isAudio = false }: Props = $props();

  function formatFileSize(size: number): string {
    if (size < 1024) return `${size} B`;
    if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
    return `${(size / (1024 * 1024)).toFixed(1)} MB`;
  }

  function handleClick() {
    window.open(attachment.storage_key, "_blank");
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      handleClick();
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="flex min-w-48 cursor-pointer items-center gap-2.5"
  role="button"
  tabindex="0"
  onclick={handleClick}
  onkeydown={handleKeyDown}
>
  <div class="rounded-full bg-(--gray-4) p-3">
    {#if attachment.content_type.startsWith("image/")}
      <img
        src={attachment.storage_key}
        alt={attachment.filename}
        class="h-10 w-10 rounded-md object-cover"
      />
    {:else if isAudio}
      <Play class="h-5 w-5 text-white" fill="white" />
    {:else}
      <File class="h-5 w-5 text-white" />
    {/if}
  </div>
  <div class="flex min-w-0 flex-col overflow-hidden">
    <div class="truncate text-sm font-medium text-(--color-text)">
      {attachment.filename}
    </div>
    <div class="text-xs text-(--color-text) opacity-70">
      {formatFileSize(attachment.size)}
    </div>
  </div>
</div>
