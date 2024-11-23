import { twitchAuthState } from "$lib/globalStores";
import { listen } from "@tauri-apps/api/event";
import { queryClient } from "./utils";
import { RUNTIME_APP_DATA_KEY } from "./runtimeAppData";
import { type RuntimeAppData } from "./types";
import { IS_AUTHENTICATED_KEY } from "./oauth";

// Handle authenticating
listen("authenticated", () => {
  queryClient.cancelQueries({ queryKey: IS_AUTHENTICATED_KEY });
  queryClient.setQueryData(IS_AUTHENTICATED_KEY, true);
});

// Handle logout
listen("logout", () => {
  queryClient.cancelQueries({ queryKey: IS_AUTHENTICATED_KEY });
  queryClient.setQueryData(IS_AUTHENTICATED_KEY, false);
});

// Update the runtime app data when the change event is received
listen<RuntimeAppData>("runtime_app_data_changed", (runtimeAppData) => {
  queryClient.cancelQueries({ queryKey: RUNTIME_APP_DATA_KEY });
  queryClient.setQueryData(RUNTIME_APP_DATA_KEY, runtimeAppData);
});
