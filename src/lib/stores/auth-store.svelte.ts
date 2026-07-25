import type { AppError, User } from "@/lib/types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type AuthStatus =
  | { type: "initializing" }
  | { type: "loading_user" }
  | { type: "authenticated"; user: User }
  | { type: "unauthenticated" }
  | { type: "network_error" }
  | { type: "server_error"; message: string };

function createAuthStore() {
  let status = $state<AuthStatus>({ type: "initializing" });
  let error = $state<AppError | null>(null);

  listen<AuthStatus>("auth:status-changed", (event) => {
    console.log("Event 'auth:status-changed' is called", event.payload);
    status = event.payload;
    error = null;
  });

  function initAuth() {
    invoke("auth_init").catch((e) => {
      console.error("auth_init failed", e);
      error = e as AppError;
    });
  }

  async function login(username: string, password: string): Promise<boolean> {
    error = null;
    try {
      await invoke("auth_login", { username, password });
      return true;
    } catch (e) {
      error = e as AppError;
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
      error = e as AppError;
      return false;
    }
  }

  async function logout(): Promise<void> {
    try {
      await invoke("auth_logout");
    } catch (e) {
      error = e as AppError;
      console.error("logout failed", e);
    }
  }

  return {
    get status() {
      return status;
    },
    get error() {
      return error;
    },
    initAuth,
    login,
    register,
    logout,
  };
}

export const authStore = createAuthStore();
