import type { Message } from "@/types";

const META_TYPES = new Set([
  "MEMBER_ADD",
  "MEMBER_REMOVE",
  "MEMBER_LEAVE",
  "CHAT_NAME_UPDATE",
  "CHAT_IMAGE_UPDATE",
  "CHAT_CREATE",
]);

export function isMetaMessage(message: Message): boolean {
  return META_TYPES.has(message.type);
}

export function shouldGroup(curr: Message, prev: Message): boolean {
  if (isMetaMessage(curr) || isMetaMessage(prev)) return false;
  if (curr.author_id !== prev.author_id) return false;
  const diff = new Date(curr.created_at).getTime() - new Date(prev.created_at).getTime();
  return diff < 5 * 60 * 1000;
}

export function groupMessagesByAuthor(messages: Message[]): Message[][] {
  if (messages.length === 0) return [];
  const groups: Message[][] = [];
  let currentGroup = [messages[0]];
  for (let i = 1; i < messages.length; i++) {
    if (shouldGroup(messages[i], messages[i - 1])) {
      currentGroup.push(messages[i]);
    } else {
      groups.push(currentGroup);
      currentGroup = [messages[i]];
    }
  }
  groups.push(currentGroup);
  return groups;
}

export function groupMessagesByDate(messages: Message[]): [string, Message[]][] {
  const map = new Map<string, Message[]>();
  for (const msg of messages) {
    const key = new Date(msg.created_at).toDateString();
    const group = map.get(key) ?? [];
    group.push(msg);
    map.set(key, group);
  }
  return Array.from(map.entries());
}

export function formatMessageTime(iso: string): string {
  return new Date(iso).toLocaleTimeString("en-US", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function formatDateSeparator(iso: string): string {
  const date = new Date(iso);
  const now = new Date();
  const isToday = date.toDateString() === now.toDateString();
  if (isToday) return "Today";
  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);
  if (date.toDateString() === yesterday.toDateString()) return "Yesterday";
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
