import { BACKEND_HTTP } from "../constants";
import { executeInterval, loadAudio } from "../utils";
import { requestHotkeys, triggerHotkey } from "../vtube-studio/hotkeys";
import { ModelParameters } from "../vtube-studio/model";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import {
  loadThrowableResources,
  throwItemMany,
} from "../vtube-studio/throw-item";
import { updateRuntimeData } from "./api";
import { beginCalibrationStep } from "./calibration";
import { CalibrationStep } from "./calibration-types";
import { AppData, SoundConfig, ThrowableConfig } from "./types";

export type EventSourceData = {
  appData: AppData;
  vtSocket: VTubeStudioWebSocket | undefined;
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

async function onMessage(data: EventSourceData, event: any) {
  switch (event.type) {
    case "SetCalibrationStep": {
      if (data.vtSocket) {
        onSetCalibrationStepEvent(data.vtSocket, event.step);
      }
      break;
    }
    case "ThrowItem": {
      if (data.vtSocket && data.modelParameters) {
        onThrowItemEvent(
          data.appData,
          data.vtSocket,
          data.modelParameters,
          event.config,
          event.amount
        );
      }

      break;
    }

    case "ThrowItemBarrage": {
      if (data.vtSocket && data.modelParameters) {
        onThrowItemBarrageEvent(
          data.appData,
          data.vtSocket,
          data.modelParameters,
          event.configs,
          event.amount_per_throw,
          event.amount,
          event.frequency
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
        onPlaySoundEvent(event.config);
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

async function onPlaySoundEvent(config: SoundConfig) {
  const audio = await loadAudio(config.src);
  audio.volume = config.volume;
  audio.play();
}

async function onTriggerHotkeyEvent(
  vtSocket: VTubeStudioWebSocket,
  hotkeyID: string
) {
  const hotkeys = await triggerHotkey(vtSocket, hotkeyID);

  return hotkeys;
}

async function onSetCalibrationStepEvent(
  vtSocket: VTubeStudioWebSocket,
  step: CalibrationStep
) {
  beginCalibrationStep(vtSocket, step);
}

async function onThrowItemEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: ThrowableConfig,
  amount: number
) {
  const { image, audio } = await loadThrowableResources(
    config.image,
    config.sound
  );

  // Failed to load the image for the throwable
  if (!image) {
    return;
  }

  await throwItemMany(
    vtSocket,
    appData,
    modelParameters,
    config,
    image,
    audio,
    amount
  );
}

async function onThrowItemBarrageEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  configs: ThrowableConfig[],
  amountPerThrow: number,
  amount: number,
  frequency: number
) {
  // Load all resources
  const resources = await Promise.all(
    configs.map(async (config) => {
      const { image, audio } = await loadThrowableResources(
        config.image,
        config.sound
      );

      return { config, image, audio };
    })
  );

  await executeInterval(
    async () => {
      await Promise.all(
        resources.map(({ config, image, audio }) => {
          // Failed to load the image for the throwable
          if (!image) {
            return Promise.resolve();
          }

          return throwItemMany(
            vtSocket,
            appData,
            modelParameters,
            config,
            image,
            // Clone audio source to allow playing multiple times
            audio,
            amountPerThrow
          );
        })
      );
    },
    frequency,
    amount
  );
}
