<script lang="ts">
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Button } from "@/lib/components/ui";
</script>

<div class="flex h-screen flex-col p-4">
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
