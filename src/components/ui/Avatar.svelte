<script lang="ts">
  import { cn } from "@/utils";
  import { Avatar } from "bits-ui";

  let {
    src,
    alt,
    fallback,
    class: className,
    ...rest
  }: Avatar.RootProps & {
    src: string | null | undefined;
    alt: string;
    fallback: string;
  } = $props();

  let rootClasses = $derived(
    cn(
      "bg-(--gray-3) text-(--color-text-muted) h-12 w-12 rounded-full border text-[20px] font-medium uppercase",
      className,
    ),
  );
</script>

<Avatar.Root delayMs={200} class={rootClasses} {...rest}>
  <div class="flex h-full w-full items-center justify-center overflow-hidden rounded-full">
    <Avatar.Image {src} {alt} />
    <Avatar.Fallback>
      {fallback}
    </Avatar.Fallback>
  </div>
</Avatar.Root>
