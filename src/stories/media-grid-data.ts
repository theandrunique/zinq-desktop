import type { ImageMeta } from "@/components/MediaGrid/MediaGrid.svelte";

function img(w: number, h: number): ImageMeta {
  return {
    src: `https://placehold.co/${w}x${h}`,
    width: w,
    height: h,
  };
}

export const imageSets: { label: string; images: ImageMeta[] }[] = [
  {
    label: "1 image",
    images: [img(1200, 800)],
  },
  {
    label: "2 images",
    images: [img(1900, 600), img(3000, 1500)],
  },
  {
    label: "3 images",
    images: [img(800, 600), img(600, 600), img(600, 1200)],
  },
  {
    label: "4 images",
    images: [img(800, 600), img(600, 800), img(1200, 800), img(500, 500)],
  },
  {
    label: "5 images",
    images: [img(600, 800), img(800, 600), img(500, 500), img(1200, 800), img(400, 600)],
  },
  {
    label: "6 images",
    images: [
      img(800, 600), img(600, 800), img(500, 700),
      img(1200, 800), img(700, 500), img(600, 600),
    ],
  },
  {
    label: "7 images",
    images: [
      img(800, 600), img(600, 800), img(500, 500),
      img(1200, 800), img(400, 600), img(700, 500),
      img(900, 600),
    ],
  },
  {
    label: "8 images",
    images: [
      img(800, 600), img(600, 800), img(500, 500),
      img(1200, 800), img(400, 600), img(700, 500),
      img(900, 600), img(800, 600),
    ],
  },
  {
    label: "9 images",
    images: [
      img(800, 600), img(600, 800), img(500, 500),
      img(1200, 800), img(400, 600), img(700, 500),
      img(900, 600), img(800, 600), img(600, 800),
    ],
  },
  {
    label: "10 images",
    images: [
      img(800, 600), img(600, 800), img(500, 500),
      img(1200, 800), img(400, 600), img(700, 500),
      img(900, 600), img(800, 600), img(600, 800),
      img(500, 500),
    ],
  },
];
