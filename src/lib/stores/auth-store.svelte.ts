import type { TauriAppError } from "@/types";
import type { User } from "@lucide/svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type AuthStatus =
  | "initializing"
  | "refreshing"
  | "loading_user"
  | "authenticated"
  | "unauthenticated";

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

    invoke("auth_init").catch((e) => {
      console.error("auth_init failed", e);
      error = e as TauriAppError;
    });
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

  async function logout(): Promise<void> {
    try {
      await invoke("auth_logout");
    } catch (e) {
      error = e as TauriAppError;
      console.error("logout failed", e);
    }
    status = "unauthenticated";
    user = null;
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
