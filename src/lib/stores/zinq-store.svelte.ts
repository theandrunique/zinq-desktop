import { invoke } from "@tauri-apps/api/core";

function createZinqStore() {

  function init() {
    invoke("zinq_init").catch((e) => {
      console.error("zinq_init failed", e);
    });
  }

  return {
    init,
  };
}

export const zinqStore = createZinqStore();
