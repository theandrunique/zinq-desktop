import { ChannelType, type Chat } from "@/types";
import { renderMessageText } from "@/utils";

export function formatTime(iso: string): string {
  const date = new Date(iso);
  const now = new Date();

  const isToday =
    date.getDate() === now.getDate() &&
    date.getMonth() === now.getMonth() &&
    date.getFullYear() === now.getFullYear();

  const startOfWeek = new Date(now.getTime());
  startOfWeek.setDate(now.getDate() - now.getDay());
  startOfWeek.setHours(0, 0, 0, 0);

  if (isToday) {
    return date.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
    });
  }
  if (date >= startOfWeek) {
    return date.toLocaleDateString("en-US", { weekday: "short" });
  }
  return date.toLocaleDateString();
}

export function getMessagePreview(chat: Chat): string | null {
  if (!chat.last_message) return null;

  const metaTypes = new Set([
    "MEMBER_ADD",
    "MEMBER_REMOVE",
    "MEMBER_LEAVE",
    "CHAT_NAME_UPDATE",
    "CHAT_IMAGE_UPDATE",
    "CHAT_CREATE",
  ]);

  if (chat.type === ChannelType.GROUP_DM && !metaTypes.has(chat.last_message.type)) {
    return `${chat.last_message.author.username}: ${renderMessageText(chat.last_message)}`;
  }
  return renderMessageText(chat.last_message);
}
