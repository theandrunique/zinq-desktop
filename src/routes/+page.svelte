<script lang="ts">
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Loader, Button } from "@/lib/components/ui";
</script>

<div class="flex h-screen flex-col items-center justify-center gap-4">
  {#if authStore.status.type === "initializing"}
    <Loader size="lg" />
    <p class="text-sm text-(--color-text-muted)">Initializing...</p>
  {:else if authStore.status.type === "loading_user"}
    <Loader size="lg" />
    <p class="text-sm text-(--color-text-muted)">Loading profile...</p>
  {:else if authStore.status.type === "network_error"}
    <p class="text-sm text-(--color-text-muted)">Network error</p>
    <Button onclick={() => authStore.initAuth()}>Retry again</Button>
  {:else if authStore.status.type === "server_error"}
    <p class="text-sm text-(--color-text-muted)">
      {authStore.status.message}
    </p>
    <Button onclick={() => authStore.initAuth()}>Retry again</Button>
  {/if}
</div>
