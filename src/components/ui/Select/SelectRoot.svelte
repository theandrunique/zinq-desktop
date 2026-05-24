<script lang="ts">
  import { Select, type WithoutChild } from "bits-ui";
  import { cn } from "@/utils";
  import { ChevronDown } from "@lucide/svelte";
  import { Check } from "@lucide/svelte";

  type Props = WithoutChild<Select.RootProps> & {
    placeholder?: string;
    items: { value: string; label: string; disabled?: boolean }[];
  };
  let { value = $bindable(), items, placeholder, open, ...rest }: Props = $props();

  let triggerClasses = $derived(
    cn(
      "flex items-center justify-between w-full p-1 px-2 border border-[var(--color-border)] rounded-[var(--radius)] text-sm bg-[var(--color-bg-input)] text-[var(--color-text)] cursor-pointer",
      "hover:bg-[var(--color-bg-input-hover)]",
      "focus:outline-[2px] focus:outline-[var(--color-focus)] focus:outline-offset-0",
      "data-[state=open]:outline-[2px] data-[state=open]:outline-[var(--color-focus)] data-[state=open]:outline-offset-0",
      "data-[state=open]:[&_svg]:rotate-180",
      "disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-[var(--gray-3)]",
      "focus:border-[var(--color-focus)]",
      "data-[placeholder]:text-[var(--color-text-muted)] data-[placeholder]:font-semibold",
    ),
  );

  let contentClasses = $derived(
    cn(
      "select-content z-50 max-h-[--bits-select-content-available-height] rounded-md p-1 min-w-(--bits-select-anchor-width)",
      "bg-[var(--color-bg)] shadow-[0_8px_30px_rgb(0,0,0,0.8)]",
      "origin-[--bits-select-content-transform-origin]",
    ),
  );

  let itemClasses = $derived(
    cn(
      "relative flex justify-between cursor-pointer select-none items-center rounded-[var(--radius)] px-1 py-1 text-sm text-[var(--color-text)] outline-none hover:bg-[var(--color-interactive-hover)] focus:bg-[var(--color-interactive-hover)] data-[highlighted]:bg-[var(--color-interactive-hover)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
    ),
  );
</script>

<Select.Root bind:value={value as never} {items} {...rest}>
  <Select.Trigger class={triggerClasses}>
    <Select.Value {placeholder} />
    <span
      class="shrink-0 text-(--color-text-muted) transition-transform duration-200"
      class:rotate-180={open}
    >
      <ChevronDown class="h-4 w-4" />
    </span>
  </Select.Trigger>

  <Select.Portal>
    <Select.Content class={contentClasses} sideOffset={4} collisionPadding={5}>
      {#each items as { value, label, disabled } (value)}
        <Select.Item {value} {label} {disabled} class={itemClasses}>
          {#snippet children({ selected })}
            {label}
            {#if selected}
              <Check class="h-4 w-4" />
            {/if}
          {/snippet}
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  @keyframes enter {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
  }
  @keyframes exit {
    to {
      opacity: 0;
      transform: scale(0.95);
    }
  }

  :global(.select-content[data-state="open"]) {
    animation: enter 150ms ease-out;
  }

  :global(.select-content[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
