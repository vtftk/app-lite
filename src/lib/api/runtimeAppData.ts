import {
  createMutation,
  createQuery,
  type CreateQueryResult,
} from "@tanstack/svelte-query";
import type { AppData, ItemConfig, RuntimeAppData } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { getContext } from "svelte";
import { derived, get, type Readable } from "svelte/store";
import { queryClient } from "./utils";

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

export function createAppDateMutation() {
  return createMutation<boolean, Error, AppData>({
    mutationFn: (appData) => invoke<boolean>("set_app_data", { appData }),
    onMutate: async (appData) => {
      queryClient.cancelQueries({ queryKey: APP_DATA_KEY });
      queryClient.setQueryData(APP_DATA_KEY, appData);
      return appData;
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: APP_DATA_KEY });
    },
  });
}

/**
 * Create a query to load the overlay URL
 */
export function createOverlayURLQuery() {
  return createQuery({
    queryKey: ["overlay-url"],
    queryFn: () => invoke<string>("get_overlay_url"),
  });
}

/**
 * Create a query to load the twitch URL
 */
export function createTwitchOAuthURLQuery() {
  return createQuery({
    queryKey: ["twitch-oauth-url"],
    queryFn: () => invoke<string>("get_twitch_oauth_uri"),
  });
}

/**
 * Creates a derived store that can determine if the
 * current model is calibrated, uses the active model
 * ID from the runtime app data combined with the model
 * data in app data
 *
 * @param appData App data for the model calibration data
 * @param runtimeAppData Runtime app data for the current model
 * @returns Readable for whether the current model is calibrated
 */
export function createDeriveModelCalibrated(
  appData: Readable<AppData>,
  runtimeAppData: Readable<RuntimeAppData>
): Readable<boolean> {
  return derived([appData, runtimeAppData], ([$appData, $runtimeAppData]) => {
    // No model active
    if ($runtimeAppData.model_id === null) {
      return false;
    }

    const modelData = $appData.models[$runtimeAppData.model_id];
    return modelData !== undefined;
  });
}

function mutateAppData(action: (state: AppData) => AppData) {
  queryClient.cancelQueries({ queryKey: APP_DATA_KEY });

  queryClient.setQueryData<AppData>(APP_DATA_KEY, (appData) => {
    if (appData === undefined) return undefined;
    return action(appData);
  });
}

type AppDataMutation = ReturnType<typeof createAppDateMutation>;

/**
 * Creates a mutation that is derived from mutating the app data
 *
 * @param appData App data store for the current app data value
 * @param appDataMutation Underlying app data mutation to use
 * @param action Action to perform the mutation
 * @returns The derived mutations
 */
export function createDerivedAppDataMutation<V>(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation,
  action: (appData: AppData, value: V) => AppData
) {
  return derived([appData, appDataMutation], ([$appData, $appDataMutation]) =>
    createMutation<boolean, Error, V>({
      mutationFn: (input) => {
        const applied = action($appData, input);
        return $appDataMutation.mutateAsync(applied);
      },
    })
  );
}

type AppDataMutator<V> = (input: V) => Promise<boolean>;

export function createAppDataMutator<V>(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation,
  action: (appData: AppData, value: V) => AppData
): Readable<AppDataMutator<V>> {
  return derived(
    appDataMutation,
    ($appDataMutation) => (input: V) =>
      $appDataMutation.mutateAsync(action(get(appData), input))
  );
}

export function createDeleteItemsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<string[]>(
    appData,
    appDataMutation,
    (appData, itemIds) => ({
      ...appData,
      items: appData.items.filter((item) => !itemIds.includes(item.id)),
    })
  );
}

type AddImpactSounds = {
  // The ID's of the items to add sounds to
  itemIds: string[];
  // The ID's of the sounds to add
  impactSoundIds: string[];
};

export function createAddImpactSounds(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<AddImpactSounds>(
    appData,
    appDataMutation,
    (appData, { itemIds, impactSoundIds }) => {
      const appendSoundsUnique = (sounds: string[]) => [
        ...sounds,
        // Add new sounds filtering out ones that are already present
        ...impactSoundIds.filter((id) => !sounds.includes(id)),
      ];

      return {
        ...appData,
        items: appData.items.map((item) => {
          if (itemIds.includes(item.id)) {
            return {
              ...item,
              impact_sounds_ids: appendSoundsUnique(item.impact_sounds_ids),
            };
          }

          return item;
        }),
      };
    }
  );
}

export function createDeleteScriptsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<string[]>(
    appData,
    appDataMutation,
    (appData, scriptIds) => ({
      ...appData,
      scripts: appData.scripts.filter(
        (script) => !scriptIds.includes(script.id)
      ),
    })
  );
}
