<script lang="ts">
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Button } from "@/components/ui";

  function handleLogout() {
    authStore.logout();
  }
</script>

<div class="flex h-screen flex-col p-4">
  <div class="flex items-center justify-between">
    <h1 class="text-lg font-semibold">Chats</h1>
    {#if authStore.user}
      <div class="flex items-center gap-3">
        <span class="text-sm text-(--color-text-muted)">
          {authStore.user.username}
        </span>
        <Button variant="secondary" onclick={handleLogout}>Logout</Button>
      </div>
    {/if}
  </div>
</div>
