<script lang="ts">
  import { MessageList } from "@/components/MessageList";
  import { Button } from "@/components/ui";
  import { testMessages } from "@/stories/test-data";
</script>

<div class="flex flex-col h-screen">
  <div><Button href="/storybook">Back</Button></div>

  <div
    class="overflow-y-auto grow"
  >
    <MessageList messages={testMessages} currentUserId="user_self" maxReadMessageId="msg_t5" />
  </div>
</div>
