import { createQuery, type CreateQueryResult } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export const IS_AUTHENTICATED_KEY = ["is-authenticated"];

/**
 * Create a query to fetch the runtime app data
 */
export function createIsAuthenticatedQuery(): CreateQueryResult<
  boolean,
  Error
> {
  return createQuery({
    queryKey: IS_AUTHENTICATED_KEY,
    queryFn: () => invoke<boolean>("is_authenticated"),
  });
}
