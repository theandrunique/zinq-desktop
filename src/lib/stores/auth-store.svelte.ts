import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type AuthStatus =
  | "initializing"
  | "refreshing"
  | "loading_user"
  | "authenticated"
  | "unauthenticated";

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

export type SessionLifetime =
  | "WEEK"
  | "MONTH"
  | "MONTH_3"
  | "MONTH_6"
  | "MONTH_12";

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

interface AuthEventPayload {
  status: AuthStatus;
  user?: User;
}

function createAuthStore() {
  let status = $state<AuthStatus>("initializing");
  let user = $state<User | null>(null);
  let error = $state<TauriAppError | null>(null);

  function initAuth() {
    listen<AuthEventPayload>("auth:status-changed", (event) => {
      status = event.payload.status;
      user = event.payload.user ?? null;
      error = null;
    });

    invoke("auth_init");
  }

  async function login(username: string, password: string): Promise<boolean> {
    error = null;
    try {
      await invoke("auth_login", { username, password });
      return true;
    } catch (e) {
      error = e as TauriAppError;
      return false;
    }
  }

  async function register(
    username: string,
    email: string,
    global_name: string,
    password: string,
  ): Promise<boolean> {
    error = null;
    try {
      await invoke("auth_register", { username, email, globalName: global_name, password });
      return true;
    } catch (e) {
      error = e as TauriAppError;
      return false;
    }
  }

  function logout() {
    invoke("auth_logout");
    status = "unauthenticated";
    user = null;
    error = null;
  }

  return {
    get status() {
      return status;
    },
    get user() {
      return user;
    },
    get error() {
      return error;
    },
    get isAuthenticated() {
      return status === "authenticated";
    },
    get isInitializing() {
      return status === "initializing";
    },
    initAuth,
    login,
    register,
    logout,
  };
}

export const authStore = createAuthStore();
