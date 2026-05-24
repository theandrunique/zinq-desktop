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
  content: string;
  created_at: string;
  edited_at: string | null;
  attachments: Attachment[];
}

export type Message = MessageBase & (
  | { type: "DEFAULT"; metadata: null }
  | { type: "REPLY"; metadata: { referenced_message_id: string } }
  | { type: "MEMBER_ADD"; metadata: { user_id: string } }
  | { type: "MEMBER_REMOVE"; metadata: { user_id: string } }
  | { type: "MEMBER_LEAVE"; metadata: { user_id: string } }
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
  lastMessage: Message | null;
  members: ChatMember[];
}

export interface ChatMember {
  user_id: string;
  username: string | null;
  global_name: string | null;
  avatar: string | null;
}
