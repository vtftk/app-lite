import { invoke } from "@tauri-apps/api/core";
import { createQuery, type CreateQueryResult } from "@tanstack/svelte-query";

import type { CustomReward } from "./types";

import { queryClient } from "./utils";

export const GET_REDEEMS_LIST_KEY = ["get-redeems-list"];

/**
 * Create a query to fetch the runtime app data
 */
export function createGetRedeemsList(): CreateQueryResult<
  CustomReward[],
  Error
> {
  return createQuery({
    queryKey: GET_REDEEMS_LIST_KEY,
    queryFn: () => invoke<CustomReward[]>("get_redeems_list"),
  });
}

export async function refreshRedeemsList() {
  await invoke<boolean>("refresh_redeems_list");

  queryClient.cancelQueries({ queryKey: GET_REDEEMS_LIST_KEY });
  queryClient.invalidateQueries({ queryKey: GET_REDEEMS_LIST_KEY });
}
