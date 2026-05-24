import type { Message } from "@/types";
import type { ClassValue } from "clsx";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const renderMessageText = (message: Message) => {
  if (message.type === "DEFAULT" || message.type === "REPLY") {
    return message.content;
  }
  if (message.type === "MEMBER_ADD") {
    return `${message.author.username} added ${message.metadata.user.username}`;
  } else if (message.type === "MEMBER_REMOVE") {
    return `${message.author.username} removed ${message.metadata.user.username}`;
  } else if (message.type === "MEMBER_LEAVE") {
    return `${message.author.username} left the chat`;
  } else if (message.type === "CHAT_NAME_UPDATE") {
    return `${message.author.username} changed chat name to "${message.metadata.new_name}"`;
  } else if (message.type === "CHAT_IMAGE_UPDATE") {
    return `${message.author.username} updated chat image`;
  } else if (message.type === "CHAT_CREATE") {
    return `${message.author.username} created the group "${message.metadata.chat_name}"`;
  } else {
    return `Unsupported message type ${message.type}. Update the app to support this message type.`;
  }
};
