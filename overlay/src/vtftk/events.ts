import { BACKEND_EVENTS } from "../constants";
import { loadAudio } from "../utils";
import { requestHotkeys, triggerHotkey } from "../vtube-studio/hotkeys";
import { ModelParameters } from "../vtube-studio/model";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { loadThrowableResources, throwItem } from "../vtube-studio/throw-item";
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
  const eventSource = new EventSource(BACKEND_EVENTS);
  eventSource.onopen = () => {
    console.debug("listening to events");
  };
  eventSource.onmessage = (msg) => {
    const event = JSON.parse(msg.data);

    switch (event.type) {
      case "SetCalibrationStep": {
        if (data.vtSocket) {
          onSetCalibrationStepEvent(data.vtSocket, event.step);
        }
        break;
      }
      case "Throw": {
        if (data.vtSocket && data.modelParameters) {
          onThrowEvent(
            data.appData,
            data.vtSocket,
            data.modelParameters,
            event.config
          );
        }

        break;
      }

      case "ThrowMany": {
        if (data.vtSocket && data.modelParameters) {
          onThrowManyEvent(
            data.appData,
            data.vtSocket,
            data.modelParameters,
            event.config,
            event.amount
          );
        }
        break;
      }

      case "ThrowDifferent": {
        if (data.vtSocket && data.modelParameters) {
          onThrowDifferentEvent(
            data.appData,
            data.vtSocket,
            data.modelParameters,
            event.configs
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
          onPlaySoundEvent(event.config, event.delay);
        }

        break;
      }
    }
  };

  eventSource.onerror = (event) => {
    console.error(event);
  };

  return eventSource;
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

async function onPlaySoundEvent(config: SoundConfig, delay: number) {
  const audio = await loadAudio(config.src);
  audio.volume = config.volume;

  setTimeout(() => audio.play(), delay);
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

async function onThrowEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: ThrowableConfig
) {
  const { image, audio } = await loadThrowableResources(
    config.image,
    config.sound
  );

  // Failed to load the image for the throwable
  if (!image) {
    return;
  }

  throwItem(vtSocket, appData, modelParameters, config, image, audio);
}

async function onThrowManyEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: ThrowableConfig,
  amount: number = 1
) {
  const { image, audio } = await loadThrowableResources(
    config.image,
    config.sound
  );

  // Failed to load the image for the throwable
  if (!image) {
    return;
  }

  await Promise.all(
    Array.from(Array(amount)).map(() =>
      throwItem(vtSocket, appData, modelParameters, config, image, audio)
    )
  );
}

async function onThrowDifferentEvent(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  configs: ThrowableConfig[]
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

  await Promise.all(
    resources.map(({ config, image, audio }) => {
      // Failed to load the image for the throwable
      if (!image) {
        return Promise.resolve();
      }

      return throwItem(
        vtSocket,
        appData,
        modelParameters,
        config,
        image,
        audio
      );
    })
  );
}
