<script lang="ts">
  import { Tooltip } from "bits-ui";
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import "./main.css";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  let { children } = $props();

  $effect(() => {
    authStore.initAuth();
  });

  $effect(() => {
    if (authStore.status.type === "authenticated") {
      goto(resolve("/chats"));
    } else if (authStore.status.type === "unauthenticated") {
      goto(resolve("/auth/login"));
    }
  });
</script>

<Tooltip.Provider delayDuration={500}>
  {@render children()}
</Tooltip.Provider>
