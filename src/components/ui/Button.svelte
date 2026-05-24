<script lang="ts">
  import { Button } from "bits-ui";
  import { cva, type VariantProps } from "class-variance-authority";
  import { cn } from "@/utils";

  const buttonVariants = cva("", {
    variants: {
      variant: {
        primary:
          "bg-[var(--color-accent)] enabled:hover:bg-[var(--color-accent-hover)] enabled:active:bg-[var(--color-accent-active)]",
        secondary:
          "bg-[var(--color-button-secondary)] enabled:hover:bg-[var(--color-interactive-hover)] enabled:active:bg-[var(--color-interactive-active)]",
        icon: "bg-transparent enabled:hover:bg-[var(--color-interactive-hover)] enabled:active:bg-[var(--color-interactive-active)] p-1",
        destructive: "bg-red-600 enabled:hover:bg-red-700 enabled:active:bg-red-800",
      },
    },
    defaultVariants: {
      variant: "primary",
    },
  });

  type Variant = VariantProps<typeof buttonVariants>["variant"];
  type Props = Button.RootProps & {
    variant?: Variant;
  };

  let { variant = "primary", class: className = "", children, ...rest }: Props = $props();

  let classes = $derived(
    cn(
      "px-2 py-1 text-sm font-semibold rounded-[var(--radius)] text-[var(--color-text)] disabled:opacity-50 disabled:cursor-not-allowed",
      buttonVariants({ variant }),
      className,
    ),
  );
</script>

<Button.Root class={classes} {...rest}>
  {@render children?.()}
</Button.Root>
