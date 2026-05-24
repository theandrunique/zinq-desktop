<script lang="ts">
  import { Slider } from "bits-ui";
  import { cn } from "@/utils";

  type Props = {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
    class?: string;
  };

  let {
    value = $bindable(0),
    min = 0,
    max = 100,
    step = 1,
    disabled = false,
    class: className,
  }: Props = $props();

  let rangeClasses = $derived(
    cn("absolute h-full bg-[var(--color-accent)] rounded-full", className),
  );

  let thumbClasses = $derived(
    cn(
      "block w-4 h-4 bg-white rounded-full",
      "focus:outline-none focus:ring-2 focus:ring-[var(--color-focus)] focus:ring-offset-1 focus:ring-offset-[var(--color-bg)]",
      "data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed",
    ),
  );
</script>

<Slider.Root
  bind:value
  {min}
  {max}
  {step}
  {disabled}
  type="single"
  class="relative flex w-full items-center"
>
  <div class="relative h-1.5 w-full rounded-full bg-[var(--gray-7)]">
    <Slider.Range class={rangeClasses} />
  </div>
  <Slider.Thumb index={0} class={thumbClasses} />
</Slider.Root>
