<script lang="ts">
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Button } from "@/lib/components/ui";
  import { networkStatusStore } from "@/lib/stores/network-store.svelte";
</script>

<div class="flex h-screen flex-col p-4">

  <div class="flex justify-center items-center gap-3">
    <div class={`rounded-full p-1 ${networkStatusStore.status.is_online ? "bg-green-500" : "bg-red-500"}`}></div>
    <div>{networkStatusStore.status.last_ping_ms} ms</div>
  </div>

  <div class="flex items-center justify-center">
    {#if authStore.status.type === "authenticated"}
      <div class="flex flex-col items-center gap-3">
        <div class="text-sm text-(--color-text-muted)">
          {JSON.stringify(authStore.status.user)}
        </div>
        <Button variant="secondary" onclick={() => authStore.logout()}>Logout</Button>
      </div>
    {/if}
  </div>
</div>
