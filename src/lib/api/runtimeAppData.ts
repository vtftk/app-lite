import { getContext } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { get, derived, type Readable } from "svelte/store";
import {
  createQuery,
  createMutation,
  type CreateQueryResult,
} from "@tanstack/svelte-query";

import type {
  AppData,
  ModelData,
  MainConfig,
  ModelConfig,
  SoundsConfig,
  PhysicsConfig,
  RuntimeAppData,
  ExternalsConfig,
  ThrowablesConfig,
  VTubeStudioConfig,
} from "./types";

import { queryClient } from "./client";

export const RUNTIME_APP_DATA_KEY = ["runtime-app-data"];
export const RUNTIME_APP_DATA_CONTEXT = Symbol("runtime-app-data");

export const APP_DATA_KEY = ["app-data"];
export const APP_DATA_CONTEXT = Symbol("app-data");

// Update the runtime app data when the change event is received
listen<RuntimeAppData>("runtime_app_data_changed", ({ payload }) => {
  queryClient.cancelQueries({ queryKey: RUNTIME_APP_DATA_KEY });
  queryClient.setQueryData(RUNTIME_APP_DATA_KEY, payload);
});

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

export function getTwitchOAuthURI() {
  return invoke<string>("get_twitch_oauth_uri");
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
  runtimeAppData: Readable<RuntimeAppData>,
): Readable<boolean> {
  return derived(
    [modelData, runtimeAppData],
    ([$modelData, $runtimeAppData]) => {
      console.log($runtimeAppData.model_id, $modelData);

      // No model active
      if ($runtimeAppData.model_id === null) {
        return false;
      }

      const data = $modelData.find(
        (data) => data.id === $runtimeAppData.model_id,
      );
      return data !== undefined;
    },
  );
}

type AppDataMutation = ReturnType<typeof createAppDateMutation>;
type AppDataMutator<V> = (input: V) => Promise<boolean>;

export function createAppDataMutator<V>(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation,
  action: (appData: AppData, value: V) => AppData,
): Readable<AppDataMutator<V>> {
  return derived(
    appDataMutation,
    ($appDataMutation) => (input: V) =>
      $appDataMutation.mutateAsync(action(get(appData), input)),
  );
}

type UpdateSettingsMutation = {
  throwables_config: Partial<ThrowablesConfig>;
  model_config: Partial<ModelConfig>;
  sounds_config: Partial<SoundsConfig>;
  vtube_studio_config: Partial<VTubeStudioConfig>;
  externals_config: Partial<ExternalsConfig>;
  main_config: Partial<MainConfig>;
  physics_config: Partial<PhysicsConfig>;
};

export function createUpdateSettingsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation,
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
        main_config,
        physics_config,
      },
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
      main_config: {
        ...appData.main_config,
        ...main_config,
      },
      physics_config: {
        ...appData.physics_config,
        ...physics_config,
      },
    }),
  );
}
