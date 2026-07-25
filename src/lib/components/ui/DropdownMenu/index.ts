import { DropdownMenu as BitsDropdownMenu } from "bits-ui";
import DropdownMenuContent from "./DropdownMenuContent.svelte";
import DropdownMenuItem from "./DropdownMenuItem.svelte";

export const DropdownMenu = {
  Root: BitsDropdownMenu.Root,
  Trigger: BitsDropdownMenu.Trigger,
  Portal: BitsDropdownMenu.Portal,
  Content: DropdownMenuContent,
  Item: DropdownMenuItem,
};
