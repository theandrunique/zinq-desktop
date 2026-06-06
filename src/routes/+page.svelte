<script lang="ts">
  import { goto } from "$app/navigation";
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Loader } from "@/components/ui";

  $effect(() => {
    if (authStore.status === "authenticated") {
      goto("/chats");
    } else if (authStore.status === "unauthenticated") {
      goto("/auth/login");
    }
  });
</script>

<div class="flex h-screen flex-col items-center justify-center gap-4">
  <Loader size="lg" />
  <p class="text-sm text-(--color-text-muted)">
    {#if authStore.status === "initializing"}
      Initializing...
    {:else if authStore.status === "refreshing"}
      Restoring session...
    {:else if authStore.status === "loading_user"}
      Loading profile...
    {/if}
  </p>
</div>
