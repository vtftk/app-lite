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
import { updateRuntimeData } from "./vtftk/api";
import { RuntimeAppData } from "./vtftk/types";

async function load() {
  // Tell the backend we aren't connected
  await updateRuntimeData({
    model_id: null,
    vtube_studio_connected: false,
  });

  const appData = await getAppData();

  const eventSourceData: EventSourceData = {
    appData,
    vtSocket: undefined,
    modelParameters: undefined,
  };

  const eventSource = createEventSource(eventSourceData);

  const vtSocket = new VTubeStudioWebSocket(
    appData.vtube_studio_config.host,
    appData.vtube_studio_config.port
  );

  eventSourceData.vtSocket = vtSocket;

  // Handle reporting the current app state when the event source is established
  eventSource.addEventListener("open", () => {
    reportCurrentRuntimeData(vtSocket);
  });

  // Run when the socket is connected
  vtSocket.onConnected = async () => {
    // Tell the backend we aren't connected
    updateRuntimeData({
      model_id: null,
      vtube_studio_connected: true,
    });

    console.debug("Connected to VTube studio");

    const { modelID } = await requestCurrentModel(vtSocket);

    // Tell the backend we aren't connected
    updateRuntimeData({
      model_id: modelID,
      vtube_studio_connected: true,
    });

    // Only needs to be done on initial load, can be stored until next refresh
    const inputParameters = await requestInputParameterList(vtSocket);
    const modelParameters = createModelParameters(
      inputParameters.defaultParameters
    );

    eventSourceData.modelParameters = modelParameters;
  };

  vtSocket.onDisconnect = () => {
    // Tell the backend we aren't connected
    updateRuntimeData({
      model_id: null,
      vtube_studio_connected: false,
    });
  };

  vtSocket.connect();
}

async function reportCurrentRuntimeData(vtSocket: VTubeStudioWebSocket) {
  let runtimeData: Partial<RuntimeAppData> = {};

  if (vtSocket.isConnected()) {
    runtimeData.vtube_studio_connected = true;

    try {
      const { modelID } = await requestCurrentModel(vtSocket);
      runtimeData.model_id = modelID;
    } catch (e) {
      console.error("failed to request current model");
    }
  }

  // Report current state to backend
  updateRuntimeData(runtimeData);
}

load();
