<script lang="ts">
  import { Switch } from "bits-ui";
  import { cn } from "@/utils";

  let { checked = $bindable(false), class: className, ...rest }: Switch.RootProps = $props();

  let rootClasses = $derived(
    cn(
      "peer inline-flex shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors",
      "bg-[var(--gray-7)] w-9 h-5",
      "data-[state=checked]:bg-[var(--color-accent)]",
      "data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed",
      className,
    ),
  );

  let thumbClasses = $derived(
    cn(
      "pointer-events-none block rounded-full bg-white shadow-lg ring-0 transition-transform w-4 h-4",
      "data-[state=unchecked]:translate-x-[0.5px]",
      "data-[state=checked]:translate-x-[15.5px]",
    ),
  );
</script>

<Switch.Root bind:checked class={rootClasses} {...rest}>
  <Switch.Thumb class={thumbClasses} />
</Switch.Root>
