export interface TauriAppError {
  kind: "NETWORK" | "API" | "SERIALIZATION" | "UNEXPECTED";
  message: string;
  api_error?: {
    code: string;
    message: string;
    errors?: Record<string, string[]>;
    metadata?: Record<string, string>;
  };
}

export interface User {
  id: string;
  username: string;
  global_name: string;
  bio: string | null;
  avatar: string | null;
  timestamp: string;
  sessions_lifetime: SessionLifetime;
  mfa: boolean;
  email: string;
  is_email_verified: boolean;
}

export type SessionLifetime = "WEEK" | "MONTH" | "MONTH_3" | "MONTH_6" | "MONTH_12";

export enum ChannelType {
  DM = "DM",
  GROUP_DM = "GROUP_DM",
}

export interface Attachment {
  id: string;
  message_id: string;
  chat_id: string;
  filename: string;
  content_type: string;
  size: number;
  storage_key: string;
  created_at: string;
}

interface MessageBase {
  id: string;
  chat_id: string;
  author_id: string;
  author: ChatMember;
  content: string;
  created_at: string;
  edited_at: string | null;
  attachments: Attachment[];
}

export type Message = MessageBase &
  (
    | { type: "DEFAULT"; metadata: null }
    | { type: "REPLY"; metadata: { referenced_message: Message } }
    | { type: "MEMBER_ADD"; metadata: { user: ChatMember } }
    | { type: "MEMBER_REMOVE"; metadata: { user: ChatMember } }
    | { type: "MEMBER_LEAVE" }
    | { type: "CHAT_NAME_UPDATE"; metadata: { new_name: string } }
    | { type: "CHAT_IMAGE_UPDATE"; metadata: { new_image: string } }
    | { type: "CHAT_CREATE"; metadata: { chat_name: string } }
  );

export interface Chat {
  id: string;
  owner_id: string | null;
  name: string | null;
  description: string | null;
  image: string | null;
  type: ChannelType;
  last_message: Message | null;
  last_read_message_id: string;
  max_read_message_id: string;
  unread_count: number;
  members: ChatMember[];
}

export interface ChatMember {
  user_id: string;
  username: string;
  global_name: string;
  avatar: string | null;
}
