<script lang="ts">
  import type { Message } from "@/types";
  import { isMetaMessage } from "./utils";
  import DefaultMessage from "./content/DefaultMessage.svelte";
  import MetaMessage from "./content/MetaMessage.svelte";
  import UnsupportedMessage from "./content/UnsupportedMessage.svelte";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();
</script>

{#if message.type === "DEFAULT" || message.type === "REPLY"}
  <DefaultMessage {message} />
{:else if isMetaMessage(message)}
  <MetaMessage {message} />
{:else}
  <UnsupportedMessage {message} />
{/if}
