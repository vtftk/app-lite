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
import { createEventSource } from "./vtftk/events";
import { beginCalibrationStep } from "./vtftk/calibration";
import { CalibrationStep } from "./vtftk/calibration-types";
import { setRuntimeData } from "./vtftk/api";

async function load() {
  const appData = await getAppData();

  const vtSocket = new VTubeStudioWebSocket(
    appData.vtube_studio.host,
    appData.vtube_studio.port
  );

  await vtSocket.connect();

  console.debug("Connected to VTube studio");

  const { modelID } = await requestCurrentModel(vtSocket);

  const modelData = appData.models[modelID];

  // Only needs to be done on initial load, can be stored until next refresh
  const inputParameters = await requestInputParameterList(vtSocket);
  const modelParameters = createModelParameters(
    inputParameters.defaultParameters
  );

  createEventSource(appData, vtSocket, modelParameters);

  // Model is not yet calibrate
  if (modelData === undefined) {
    beginCalibrationStep(vtSocket, CalibrationStep.NotStarted);
    return;
  }
}

load();
