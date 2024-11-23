import { createQuery, type CreateQueryResult } from "@tanstack/svelte-query";
import type { AppData, RuntimeAppData } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { getContext } from "svelte";
import type { Readable } from "svelte/store";

export const RUNTIME_APP_DATA_KEY = ["runtime-app-data"];
export const RUNTIME_APP_DATA_CONTEXT = Symbol("runtime-app-data");

export const APP_DATA_KEY = ["app-data"];
export const APP_DATA_CONTEXT = Symbol("app-data");

export function getRuntimeAppData(): Readable<RuntimeAppData> {
  return getContext(RUNTIME_APP_DATA_CONTEXT);
}

export function getAppData(): Readable<AppData> {
  return getContext(APP_DATA_CONTEXT);
}

/**
 * Create a query to fetch the runtime app data
 */
export function createRuntimeAppDataQuery(): CreateQueryResult<
  RuntimeAppData,
  Error
> {
  return createQuery({
    queryKey: RUNTIME_APP_DATA_KEY,
    queryFn: () => invoke<RuntimeAppData>("get_runtime_app_data"),
  });
}

/**
 * Create a query to fetch the app data
 */
export function createAppDataQuery(): CreateQueryResult<RuntimeAppData, Error> {
  return createQuery({
    queryKey: APP_DATA_KEY,
    queryFn: () => invoke<AppData>("get_app_data"),
  });
}
