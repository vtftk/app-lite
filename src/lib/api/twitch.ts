import {
  createMutation,
  createQuery,
  type CreateQueryResult,
} from "@tanstack/svelte-query";
import type { AppData, CustomReward, RuntimeAppData } from "./types";
import { invoke } from "@tauri-apps/api/core";
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

export function refreshRedeemsList() {
  queryClient.cancelQueries({ queryKey: GET_REDEEMS_LIST_KEY });
  queryClient.invalidateQueries({ queryKey: GET_REDEEMS_LIST_KEY });
}
