import { createQuery, type CreateQueryResult } from "@tanstack/svelte-query";
import type { RuntimeAppData } from "./types";
import { invoke } from "@tauri-apps/api/core";

/**
 * Create a query to fetch the runtime app data
 */
export function createRuntimeAppDataQuery(): CreateQueryResult<
  RuntimeAppData,
  Error
> {
  return createQuery({
    queryKey: ["runtime-app-data"],
    queryFn: () => invoke<RuntimeAppData>("get_runtime_app_data"),
    retry: false,
  });
}
