<script lang="ts">
  import { Tooltip } from "bits-ui";
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import "./main.css";

  let { children } = $props();

  $effect(() => {
    authStore.initAuth();
  });
</script>

<Tooltip.Provider delayDuration={500}>
  {@render children()}
</Tooltip.Provider>
