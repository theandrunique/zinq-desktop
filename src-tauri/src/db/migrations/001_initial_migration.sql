CREATE TABLE chats (
    id TEXT PRIMARY KEY NOT NULL,
    owner_id TEXT,
    name TEXT,
    description TEXT,
    image TEXT,
    last_message_id TEXT,
    permissions TEXT,
    type TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_chats_last_message_id ON chats(last_message_id DESC);

CREATE TABLE chat_members (
    chat_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    global_name TEXT NOT NULL,
    avatar TEXT,
    permissions TEXT,
    PRIMARY KEY (chat_id, user_id),
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

CREATE TABLE messages (
    id TEXT PRIMARY KEY NOT NULL,
    chat_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    edited_at TEXT,
    type TEXT NOT NULL,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

CREATE INDEX idx_messages_chat_id ON messages(chat_id, id DESC);

CREATE TABLE attachments (
    id TEXT PRIMARY KEY NOT NULL,
    message_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    storage_key TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

CREATE TABLE sync_state (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at BIGINT NOT NULL
);

INSERT INTO sync_state (key, value, updated_at) VALUES
    ('last_event_id', '0', 0),
    ('last_full_sync', '0', 0);

CREATE TABLE event_log (
    event_id TEXT PRIMARY KEY NOT NULL,
    type TEXT NOT NULL,
    created_at TEXT NOT NULL
);
