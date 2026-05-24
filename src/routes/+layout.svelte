<script lang="ts">
  import { Tooltip } from "bits-ui";
  import "./main.css";

  let { children } = $props();
</script>

<Tooltip.Provider delayDuration={500}>
  {@render children()}
</Tooltip.Provider>
