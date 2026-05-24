<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "@/utils";

  let { children, class: className, ...rest }: Dialog.TitleProps = $props();

  let cls = $derived(cn("mb-1.5 text-base font-semibold", className));
</script>

<Dialog.Title class={cls} {...rest}>
  {@render children?.()}
</Dialog.Title>
