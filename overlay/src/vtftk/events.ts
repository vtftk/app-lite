import { BACKEND_HTTP } from "../constants";
import {
  arrayRandom,
  executeInterval,
  loadAudio,
  LoadedItemMap,
  LoadedSoundMap,
  loadItems,
  loadSounds,
} from "../utils";
import { requestHotkeys, triggerHotkey } from "../vtube-studio/hotkeys";
import { ModelParameters } from "../vtube-studio/model";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { throwItem } from "../vtube-studio/throw-item";
import { updateRuntimeData } from "./api";
import { beginCalibrationStep } from "./calibration";
import { CalibrationStep } from "./calibration-types";
import {
  AppData,
  ItemWithImpactSoundIds,
  ModelCalibration,
  ModelId,
  Sound,
  ThrowableConfig,
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
          event.config,
          event.amount,
        );
      }

      break;
    }

    case "ThrowItemBarrage": {
      if (data.vtSocket && data.modelParameters) {
        onThrowItemBarrageEvent(
          data.appData,
          data.vtSocket,
          data.modelCalibration,
          data.modelParameters,
          event.config,
          event.amount_per_throw,
          event.amount,
          event.frequency,
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
  }
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
  config: ThrowableConfig,
  amount: number,
) {
  const [loadedItems, loadedSounds] = await Promise.all([
    loadItems(config.items),
    loadSounds(config.impact_sounds),
  ]);

  await throwItemMany(
    vtSocket,
    appData,
    modelCalibration,
    modelParameters,
    config.items,
    loadedItems,
    loadedSounds,
    amount,
  );
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

async function onThrowItemBarrageEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelCalibration: Map<ModelId, ModelCalibration>,
  modelParameters: ModelParameters,
  config: ThrowableConfig,
  amountPerThrow: number,
  amount: number,
  frequency: number,
) {
  const [loadedItems, loadedSounds] = await Promise.all([
    loadItems(config.items),
    loadSounds(config.impact_sounds),
  ]);

  await executeInterval(
    async () => {
      return throwItemMany(
        vtSocket,
        appData,
        modelCalibration,
        modelParameters,
        config.items,
        loadedItems,
        loadedSounds,
        amountPerThrow,
      );
    },
    frequency,
    amount,
  );
}
