import { updateRuntimeData } from "./api";
import { BACKEND_HTTP } from "../constants";
import { beginCalibrationStep } from "./calibration";
import { CalibrationStep } from "./calibration-types";
import { ModelParameters } from "../vtube-studio/model";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { triggerHotkey, requestHotkeys } from "../vtube-studio/hotkeys";
import { throwItem, setPhysicsEngineConfig } from "../vtube-studio/throw-item";
import {
  loadAudio,
  loadItems,
  loadSounds,
  arrayRandom,
  LoadedItemMap,
  LoadedSoundMap,
  executeInterval,
} from "../utils";
import {
  Sound,
  AppData,
  ModelId,
  ItemWithSounds,
  ThrowItemConfig,
  ModelCalibration,
  ThrowItemConfigType,
  ItemWithImpactSoundIds,
} from "./types";

export type EventSourceData = {
  appData: AppData;
  vtSocket: VTubeStudioWebSocket | undefined;
  modelCalibration: Map<ModelId, ModelCalibration>;
  modelParameters: ModelParameters | undefined;
};

export function createEventSource(data: EventSourceData) {
  const eventSource = new EventSource(new URL("/events", BACKEND_HTTP));

  eventSource.addEventListener("open", () => {
    console.debug("listening to events");
  });

  eventSource.addEventListener("message", (event) => {
    const eventData = JSON.parse(event.data);
    onMessage(data, eventData);
  });

  eventSource.addEventListener("error", (event) => {
    console.error("event source error", event);
  });

  return eventSource;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
async function onMessage(data: EventSourceData, event: any) {
  switch (event.type) {
    case "SetCalibrationStep": {
      if (data.vtSocket) {
        onSetCalibrationStepEvent(data, data.vtSocket, event.step);
      }
      break;
    }
    case "ThrowItem": {
      if (data.vtSocket && data.modelParameters) {
        onThrowItemEvent(
          data.appData,
          data.vtSocket,
          data.modelCalibration,
          data.modelParameters,
          event.items,
          event.config,
        );
      }

      break;
    }

    case "UpdateHotkeys": {
      if (data.vtSocket) {
        onUpdateHotkeysEvent(data.vtSocket);
      }

      break;
    }

    case "TriggerHotkey": {
      if (data.vtSocket) {
        onTriggerHotkeyEvent(data.vtSocket, event.hotkey_id);
      }

      break;
    }

    case "PlaySound": {
      if (data.vtSocket) {
        onPlaySoundEvent(data.appData, event.config);
      }

      break;
    }

    case "PlaySoundSeq": {
      if (data.vtSocket) {
        onPlaySoundSeqEvent(data.appData, event.configs);
      }

      break;
    }

    case "AppDataUpdated": {
      onAppDataUpdatedEvent(data, event.app_data);
      break;
    }
  }
}

function onAppDataUpdatedEvent(data: EventSourceData, appData: AppData) {
  data.appData = appData;

  // Recreate the physics engine
  const { fps, reverse_gravity, gravity_multiplier } = appData.physics_config;
  setPhysicsEngineConfig({
    fps: fps,
    reverseGravity: reverse_gravity,
    gravityMultiplier: gravity_multiplier,
  });
}

async function onUpdateHotkeysEvent(vtSocket: VTubeStudioWebSocket) {
  const hotkeys = await requestHotkeys(vtSocket);

  await updateRuntimeData({
    hotkeys: hotkeys.map((hotkey) => ({
      hotkey_id: hotkey.hotkeyID,
      name: hotkey.name,
    })),
  });

  return hotkeys;
}

async function onPlaySoundEvent(appData: AppData, config: Sound) {
  const audio = await loadAudio(config.src);
  audio.volume = config.volume * appData.sounds_config.global_volume;
  audio.play();
}

async function onPlaySoundSeqEvent(appData: AppData, configs: Sound[]) {
  const sounds = await loadSounds(configs);

  for (const config of configs) {
    console.debug("Playing sound config", config);

    const soundData = sounds.get(config.id);

    if (soundData === undefined) {
      console.warn("Skipping sound config that failed to load", config);
      continue;
    }

    // Play the sound
    const audio = soundData.sound;
    audio.volume = config.volume * appData.sounds_config.global_volume;

    const completePromise = new Promise<void>((resolve, reject) => {
      audio.onended = () => resolve();
      audio.onerror = () => reject();
    });

    audio.play();

    // Wait for the sound to complete fully
    await completePromise;

    console.debug("Completed sound config", config);
  }
}

async function onTriggerHotkeyEvent(
  vtSocket: VTubeStudioWebSocket,
  hotkeyID: string,
) {
  const hotkeys = await triggerHotkey(vtSocket, hotkeyID);

  return hotkeys;
}

async function onSetCalibrationStepEvent(
  data: EventSourceData,
  vtSocket: VTubeStudioWebSocket,
  step: CalibrationStep,
) {
  beginCalibrationStep(vtSocket, step, (model_data) => {
    // Update the model data map to include the new model data
    data.modelCalibration.set(model_data.id, model_data.calibration);
  });
}

async function onThrowItemEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelCalibration: Map<ModelId, ModelCalibration>,
  modelParameters: ModelParameters,
  items: ItemWithSounds,
  config: ThrowItemConfig,
) {
  const [loadedItems, loadedSounds] = await Promise.all([
    loadItems(items.items),
    loadSounds(items.impact_sounds),
  ]);

  if (config.type === ThrowItemConfigType.All) {
    await throwItemMany(
      vtSocket,
      appData,
      modelCalibration,
      modelParameters,
      items.items,
      loadedItems,
      loadedSounds,
      config.amount,
    );
  } else if (config.type === ThrowItemConfigType.Barrage) {
    await executeInterval(
      async () => {
        return throwItemMany(
          vtSocket,
          appData,
          modelCalibration,
          modelParameters,
          items.items,
          loadedItems,
          loadedSounds,
          config.amount_per_throw,
        );
      },
      config.frequency,
      config.amount,
    );
  }
}

function pickRandomSound(
  item: ItemWithImpactSoundIds,
  sounds: LoadedSoundMap,
  clone: boolean = false,
) {
  if (item.impact_sound_ids.length > 0) {
    const randomSoundId = arrayRandom(item.impact_sound_ids);
    const audio = sounds.get(randomSoundId);
    if (audio) {
      return {
        config: audio.config,
        sound: clone
          ? (audio.sound.cloneNode() as HTMLAudioElement)
          : audio.sound,
      };
    }
  }

  return null;
}

function pickRandomItem(
  items: ItemWithImpactSoundIds[],
  images: LoadedItemMap,
) {
  if (items.length === 1) {
    const item = items[0];
    const image = images.get(item.id);
    if (image === undefined) return null;
    return { config: item, image };
  }

  if (items.length > 0) {
    const randomItem = arrayRandom(items);
    const image = images.get(randomItem.id);
    if (image !== undefined) {
      return {
        config: randomItem,
        image,
      };
    }
  }

  return null;
}

function throwRandomItem(
  socket: VTubeStudioWebSocket,
  appData: AppData,
  modelCalibration: Map<ModelId, ModelCalibration>,
  modelParameters: ModelParameters,

  items: ItemWithImpactSoundIds[],
  loadedItems: LoadedItemMap,
  loadedSounds: LoadedSoundMap,
): Promise<void> {
  const item = pickRandomItem(items, loadedItems);

  // No item found
  if (item === null) return Promise.resolve();

  const impactAudio = pickRandomSound(item.config, loadedSounds);

  return throwItem(
    socket,
    appData,
    modelCalibration,

    modelParameters,
    item.config,
    item.image,
    impactAudio,
  );
}

async function throwItemMany(
  socket: VTubeStudioWebSocket,
  appData: AppData,
  modelCalibration: Map<ModelId, ModelCalibration>,
  modelParameters: ModelParameters,

  items: ItemWithImpactSoundIds[],
  loadedItems: LoadedItemMap,
  loadedSounds: LoadedSoundMap,
  amount: number,
) {
  if (amount === 1) {
    return throwRandomItem(
      socket,
      appData,
      modelCalibration,
      modelParameters,
      items,
      loadedItems,
      loadedSounds,
    );
  }

  return Promise.all(
    Array.from(Array(amount)).map(() =>
      throwRandomItem(
        socket,
        appData,
        modelCalibration,
        modelParameters,
        items,
        loadedItems,
        loadedSounds,
      ),
    ),
  );
}
