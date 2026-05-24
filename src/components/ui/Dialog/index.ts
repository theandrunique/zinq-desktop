import { Dialog as DialogBits } from "bits-ui";
import DialogClose from "./DialogClose.svelte";
import DialogContent from "./DialogContent.svelte";
import DialogOverlay from "./DialogOverlay.svelte";
import DialogTitle from "./DialogTitle.svelte";

export const Dialog = {
  Root: DialogBits.Root,
  Trigger: DialogBits.Trigger,
  Portal: DialogBits.Portal,
  Close: DialogClose,
  Content: DialogContent,
  Overlay: DialogOverlay,
  Title: DialogTitle,
};
