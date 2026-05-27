<script lang="ts">
  import { MessageList } from "@/components/MessageList";
  import { Button } from "@/components/ui";
  import { testMessages } from "@/stories/test-data";
</script>

<div class="flex h-screen flex-col">
  <div><Button href="/storybook">Back</Button></div>

  <div class="min-h-0 grow">
    <MessageList messages={testMessages} currentUserId="user_self" maxReadMessageId="msg_t5" />
  </div>
</div>
