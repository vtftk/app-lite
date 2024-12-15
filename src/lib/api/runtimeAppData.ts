import { getContext } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import { get, derived, type Readable } from "svelte/store";
import {
  createQuery,
  createMutation,
  type CreateQueryResult,
} from "@tanstack/svelte-query";

import { queryClient } from "./utils";
import type {
  AppData,
  ModelData,
  ModelConfig,
  SoundsConfig,
  RuntimeAppData,
  ExternalsConfig,
  ThrowablesConfig,
  VTubeStudioConfig,
} from "./types";

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
 * @param modelData Collection of model data
 * @param runtimeAppData Runtime app data for the current model
 * @returns Readable for whether the current model is calibrated
 */
export function createDeriveModelCalibrated(
  modelData: Readable<ModelData[]>,
  runtimeAppData: Readable<RuntimeAppData>
): Readable<boolean> {
  return derived(
    [modelData, runtimeAppData],
    ([$modelData, $runtimeAppData]) => {
      // No model active
      if ($runtimeAppData.model_id === null) {
        return false;
      }

      const data = $modelData.find(
        (data) => data.id === $runtimeAppData.model_id
      );
      return data !== undefined;
    }
  );
}

type AppDataMutation = ReturnType<typeof createAppDateMutation>;
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

type UpdateSettingsMutation = {
  throwables_config: Partial<ThrowablesConfig>;
  model_config: Partial<ModelConfig>;
  sounds_config: Partial<SoundsConfig>;
  vtube_studio_config: Partial<VTubeStudioConfig>;
  externals_config: Partial<ExternalsConfig>;
};

export function createUpdateSettingsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateSettingsMutation>(
    appData,
    appDataMutation,
    (
      appData,
      {
        throwables_config,
        model_config,
        sounds_config,
        vtube_studio_config,
        externals_config,
      }
    ) => ({
      ...appData,

      throwables_config: { ...appData.throwables_config, ...throwables_config },
      model_config: { ...appData.model_config, ...model_config },
      sounds_config: { ...appData.sounds_config, ...sounds_config },
      vtube_studio_config: {
        ...appData.vtube_studio_config,
        ...vtube_studio_config,
      },
      externals_config: {
        ...appData.externals_config,
        ...externals_config,
      },
    })
  );
}
