import { BACKEND_EVENTS } from "../constants";
import { ModelParameters } from "../vtube-studio/model";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { loadThrowableResources, throwItem } from "../vtube-studio/throw-item";
import { beginCalibrationStep } from "./calibration";
import { CalibrationStep } from "./calibration-types";
import { AppData, ThrowableConfig } from "./types";

export function createEventSource(
  appData: AppData,
  vtSocket: VTubeStudioWebSocket,
  modelParameters: ModelParameters
) {
  const eventSource = new EventSource(BACKEND_EVENTS);
  eventSource.onopen = () => {
    console.debug("listening to events");
  };
  eventSource.onmessage = (msg) => {
    const event = JSON.parse(msg.data);

    switch (event.type) {
      case "SetCalibrationStep": {
        onSetCalibrationStepEvent(vtSocket, event.step);
        break;
      }
      case "Throw": {
        onThrowEvent(appData, vtSocket, modelParameters, event.config);
        break;
      }

      case "ThrowMany": {
        onThrowManyEvent(
          appData,
          vtSocket,
          modelParameters,
          event.config,
          event.amount
        );
        break;
      }
    }
  };

  eventSource.onerror = (event) => {
    console.error(event);
  };
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
