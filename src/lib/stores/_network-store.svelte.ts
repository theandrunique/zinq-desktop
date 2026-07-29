import { listen } from "@tauri-apps/api/event";

export type NetworkStatus = {
  is_online: boolean;
  last_ping_ms: number;
}

function createNetworkStatusStore() {
  let status = $state<NetworkStatus>({ is_online: false, last_ping_ms: 0 });

  listen<NetworkStatus>("network:status-changed", (event) => {
    console.log("Event 'network:status-changed' is called", event.payload);
    status = event.payload;
  });

  return {
    get status() {
      return status;
    }
  }
}

export const networkStatusStore = createNetworkStatusStore();
