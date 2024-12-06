import {
  createMutation,
  createQuery,
  type CreateQueryResult,
} from "@tanstack/svelte-query";
import type {
  AppData,
  CommandConfig,
  EventConfig,
  ItemConfig,
  ModelConfig,
  RuntimeAppData,
  SoundConfig,
  SoundsConfig,
  ThrowablesConfig,
  UserScriptConfig,
  VTubeStudioConfig,
} from "./types";
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

export function createDeleteSoundsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<string[]>(
    appData,
    appDataMutation,
    (appData, soundIds) => ({
      ...appData,

      // Remove the sound itself
      sounds: appData.sounds.filter((sound) => !soundIds.includes(sound.id)),

      // Remove the sound from any associated items
      items: appData.items.map((item) => ({
        ...item,
        impact_sounds_ids: item.impact_sounds_ids.filter(
          (soundId) => !soundIds.includes(soundId)
        ),
      })),

      // TODO: Remove sound from events
    })
  );
}

type DeleteCommands = {
  commandIds: string[];
};

export function createDeleteCommandsMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<DeleteCommands>(
    appData,
    appDataMutation,
    (appData, { commandIds }) => ({
      ...appData,

      // Remove the command itself
      commands: appData.commands.filter(
        (command) => !commandIds.includes(command.id)
      ),
    })
  );
}

type UpdateItem = {
  /// ID of the item to update
  itemId: string;
  /// The new item configuration
  itemConfig: Omit<ItemConfig, "id">;
};

export function createUpdateItemMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateItem>(
    appData,
    appDataMutation,
    (appData, { itemId, itemConfig }) => ({
      ...appData,

      // Replace the existing item
      items: appData.items.map((item) =>
        item.id === itemId ? { ...itemConfig, id: itemId } : item
      ),
    })
  );
}

type CreateItem = {
  itemConfig: ItemConfig;
};

export function createCreateItemMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<CreateItem>(
    appData,
    appDataMutation,
    (appData, { itemConfig }) => ({
      ...appData,

      // Add the item to the end of the items list
      items: [...appData.items, itemConfig],
    })
  );
}

type UpdateSound = {
  /// ID of the sound to update
  soundId: string;
  /// The new sound configuration
  soundConfig: Omit<SoundConfig, "id">;
};

export function createUpdateSoundMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateSound>(
    appData,
    appDataMutation,
    (appData, { soundId, soundConfig }) => ({
      ...appData,

      // Replace the existing sound
      sounds: appData.sounds.map((sound) =>
        sound.id === soundId ? { ...soundConfig, id: soundId } : sound
      ),
    })
  );
}

type CreateSound = {
  soundConfig: SoundConfig;
};

export function createCreateSoundMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<CreateSound>(
    appData,
    appDataMutation,
    (appData, { soundConfig }) => ({
      ...appData,

      // Add the sound to the end of the sounds list
      sounds: [...appData.sounds, soundConfig],
    })
  );
}

type UpdateSettingsMutation = {
  throwables_config: Partial<ThrowablesConfig>;
  model_config: Partial<ModelConfig>;
  sounds_config: Partial<SoundsConfig>;
  vtube_studio_config: Partial<VTubeStudioConfig>;
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
      { throwables_config, model_config, sounds_config, vtube_studio_config }
    ) => ({
      ...appData,

      throwables_config: { ...appData.throwables_config, ...throwables_config },
      model_config: { ...appData.model_config, ...model_config },
      sounds_config: { ...appData.sounds_config, ...sounds_config },
      vtube_studio_config: {
        ...appData.vtube_studio_config,
        ...vtube_studio_config,
      },
    })
  );
}

type UpdateScript = {
  /// ID of the script to update
  scriptId: string;
  /// The new script configuration
  scriptConfig: Omit<UserScriptConfig, "id">;
};

export function createUpdateScriptMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateScript>(
    appData,
    appDataMutation,
    (appData, { scriptId, scriptConfig }) => ({
      ...appData,

      // Replace the existing script
      scripts: appData.scripts.map((script) =>
        script.id === scriptId ? { ...scriptConfig, id: scriptId } : script
      ),
    })
  );
}

type CreateScript = {
  scriptConfig: UserScriptConfig;
};

export function createCreateScriptMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<CreateScript>(
    appData,
    appDataMutation,
    (appData, { scriptConfig }) => ({
      ...appData,

      // Add the script to the end of the scripts list
      scripts: [...appData.scripts, scriptConfig],
    })
  );
}

type UpdateEvent = {
  /// ID of the event to update
  eventId: string;
  /// The new event configuration
  eventConfig: Omit<EventConfig, "id">;
};

export function createUpdateEventMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateEvent>(
    appData,
    appDataMutation,
    (appData, { eventId, eventConfig }) => ({
      ...appData,

      // Replace the existing script
      events: appData.events.map((event) =>
        event.id === eventId ? { ...eventConfig, id: eventId } : event
      ),
    })
  );
}

type CreateEvent = {
  eventConfig: EventConfig;
};

export function createCreateEventMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<CreateEvent>(
    appData,
    appDataMutation,
    (appData, { eventConfig }) => ({
      ...appData,

      // Add the script to the end of the scripts list
      events: [...appData.events, eventConfig],
    })
  );
}

type UpdateCommand = {
  /// ID of the command to update
  commandId: string;
  /// The new command configuration
  commandConfig: Omit<CommandConfig, "id">;
};

export function createUpdateCommandMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<UpdateCommand>(
    appData,
    appDataMutation,
    (appData, { commandId, commandConfig }) => ({
      ...appData,

      // Replace the existing command
      commands: appData.commands.map((event) =>
        event.id === commandId ? { ...commandConfig, id: commandId } : event
      ),
    })
  );
}

type CreateCommand = {
  commandConfig: CommandConfig;
};

export function createCreateCommandMutation(
  appData: Readable<AppData>,
  appDataMutation: AppDataMutation
) {
  return createAppDataMutator<CreateCommand>(
    appData,
    appDataMutation,
    (appData, { commandConfig }) => ({
      ...appData,

      // Add the script to the end of the commands list
      commands: [...appData.commands, commandConfig],
    })
  );
}
