import "./styles/app.css";
import "./vtftk/events";
import "./vtftk/calibration";

import { getAppData } from "./vtftk/appData";
import { VTubeStudioWebSocket } from "./vtube-studio/socket";
import {
  createModelParameters,
  requestCurrentModel,
  requestInputParameterList,
} from "./vtube-studio/model";
import { loadThrowableResources, throwItem } from "./vtube-studio/throw-item";
import { createEventSource } from "./vtftk/events";
import { beginCalibrationStep } from "./vtftk/calibration";
import { CalibrationStep } from "./vtftk/calibration-types";
import { ThrowableConfig } from "./vtftk/types";

async function load() {
  const appData = await getAppData();

  const vtSocket = new VTubeStudioWebSocket(
    appData.vtube_studio.host,
    appData.vtube_studio.port
  );
  await vtSocket.connect();

  console.debug("Connected to VTube studio");

  createEventSource(vtSocket);

  const { modelID } = await requestCurrentModel(vtSocket);

  const modelData = appData.models[modelID];

  // Model is not yet calibrate
  if (modelData === undefined) {
    beginCalibrationStep(vtSocket, CalibrationStep.NotStarted);
    return;
  }

  try {
    // Only needs to be done on initial load, can be stored until next refresh
    const inputParameters = await requestInputParameterList(vtSocket);
    const modelParameters = createModelParameters(
      inputParameters.defaultParameters
    );

    const throwableConfig: ThrowableConfig = {
      name: "Heart",
      image: {
        pixelate: false,
        scale: 0.5,
        src: "https://clipartcraft.com/images/transparent-hearts-tiny-3.png",
        weight: 1,
      },
      sound: null,
    };

    const { image, audio } = await loadThrowableResources(
      throwableConfig.image,
      throwableConfig.sound
    );

    // Failed to load the image for the throwable
    if (!image) {
      return;
    }

    await Promise.all(
      Array.from(Array(11)).map(() =>
        throwItem(
          vtSocket,
          appData,
          modelParameters,
          throwableConfig,
          image,
          audio
        )
      )
    );
  } catch (e) {
    console.error("failed to authorize", e);
  }
}

load();
