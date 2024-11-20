import { twitchAuthState } from "$lib/globalStores";
import { listen } from "@tauri-apps/api/event";

// Handle authenticating
listen("authenticated", () => {
  twitchAuthState.set(true);
  console.log("Authenticated");
});

// Handle logout
listen("logout", () => {
  twitchAuthState.set(false);
});
