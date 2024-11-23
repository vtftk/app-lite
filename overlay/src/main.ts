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
import { createEventSource, EventSourceData } from "./vtftk/events";
import { beginCalibrationStep } from "./vtftk/calibration";
import { CalibrationStep } from "./vtftk/calibration-types";
import { setRuntimeData } from "./vtftk/api";

async function load() {
  // Tell the backend we aren't connected
  await setRuntimeData({
    model_id: null,
    vtube_studio_connected: false,
  });

  const appData = await getAppData();

  const eventSourceData: EventSourceData = {
    appData,
    vtSocket: undefined,
    modelParameters: undefined,
  };

  createEventSource(eventSourceData);

  const vtSocket = new VTubeStudioWebSocket(
    appData.vtube_studio.host,
    appData.vtube_studio.port
  );

  eventSourceData.vtSocket = vtSocket;

  // Run when the socket is connected
  vtSocket.onConnected = async () => {
    // Tell the backend we aren't connected
    setRuntimeData({
      model_id: null,
      vtube_studio_connected: true,
    });

    console.debug("Connected to VTube studio");

    const { modelID } = await requestCurrentModel(vtSocket);

    // Tell the backend we aren't connected
    setRuntimeData({
      model_id: modelID,
      vtube_studio_connected: true,
    });

    const modelData = appData.models[modelID];

    // Only needs to be done on initial load, can be stored until next refresh
    const inputParameters = await requestInputParameterList(vtSocket);
    const modelParameters = createModelParameters(
      inputParameters.defaultParameters
    );

    eventSourceData.modelParameters = modelParameters;

    // Model is not yet calibrated
    if (modelData === undefined) {
      beginCalibrationStep(vtSocket, CalibrationStep.NotStarted);
      return;
    }
  };

  vtSocket.onDisconnect = () => {
    // Tell the backend we aren't connected
    setRuntimeData({
      model_id: null,
      vtube_studio_connected: false,
    });
  };

  vtSocket.connect();
}

load();
