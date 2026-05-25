<script lang="ts">
  import type { Message } from "@/types";

  interface Props {
    message: Message;
  }

  let { message }: Props = $props();
</script>

<div class="text-xs italic opacity-50">
  Unsupported message type: {message.type}
</div>
