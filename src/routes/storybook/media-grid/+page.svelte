<script lang="ts">
  import { MediaGrid } from "@/components/MediaGrid";
  import { Button } from "@/components/ui";
  import { imageSets } from "@/stories/media-grid-data";
</script>

<svelte:head>
  <title>MediaGrid — Storybook</title>
  <meta name="description" content="MediaGrid component showcase" />
</svelte:head>

<div class="flex flex-col gap-8 p-8">
  <div><Button href="/storybook">Back</Button></div>

  <h1 class="text-2xl font-bold">MediaGrid</h1>

  {#each imageSets as set (set.label)}
    <section class="space-y-2">
      <h2 class="text-sm font-semibold text-(--color-text-muted)">{set.label}</h2>
      <div class="max-w-[min(90%,28rem)]">
        <MediaGrid images={set.images} maxHeight={250} />
      </div>
    </section>
  {/each}
</div>
