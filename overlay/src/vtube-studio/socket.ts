import { attemptAuthorization } from "./auth";
import { APIError } from "./error";
import { flinch } from "./flinch";
import { VTubeMessage } from "./message";
import {
  createModelParameters,
  requestCurrentModel,
  requestInputParameterList,
} from "./model";
import {
  FaceConfig,
  ImageConfig,
  loadThrowable,
  ModelConfig,
  ThrowConfig,
  ThrowDirection,
  throwItem,
} from "./throw-item";

type PromiseExecutor = {
  resolve: (value: VTubeMessage<any>) => void;
  reject: (reason?: any) => void;
};

const defaultHost = "localhost";
const defaultPort = 8001;

const retryTimeout = 5 * 1000;

let host = defaultHost;
let port = defaultPort;

// Websocket, connected to VTube studio
let globalSocket: WebSocket = connect();
let retryInterval: number | undefined;

let requestID = 0;

let requestHandlers: Map<number, PromiseExecutor> = new Map();

const clearRequestHandlers = () => {
  requestHandlers.forEach((executor) => executor.reject());
  requestHandlers.clear();
};

function connect() {
  const socket = new WebSocket(`ws://${host}:${port}/`);

  socket.onopen = () => {
    clearInterval(retryInterval);
    console.debug("connected to VTube Studio");

    if (socket.readyState !== WebSocket.OPEN) return;

    setTimeout(async () => {
      try {
        await attemptAuthorization();

        console.debug("Authorization complete");

        await requestCurrentModel();

        // Only needs to be done on initial load, can be stored until next refresh
        const inputParameters = await requestInputParameterList();
        const modelParameters = createModelParameters(
          inputParameters.defaultParameters
        );

        const imageConfig: ImageConfig = {
          pixel: false,
          scale: 1,
          src: "https://clipartcraft.com/images/transparent-hearts-tiny-3.png",
          weight: 1,
        };

        const throwConfig: ThrowConfig = {
          direction: ThrowDirection.Random,
          throwAngle: { min: -360, max: 360 },
          itemScale: { min: 0.2, max: 1.2 },
          spinSpeed: { min: 5, max: 15 },
          duration: 5000,
          delay: 0,
        };

        // TODO: Should come from calibration,
        const faceConfig: FaceConfig = {
          x: { min: -0.5833332777023315, max: -0.5833332777023315 },
          y: { min: -0.8053247107399835, max: -0.8053247107399835 },
        };

        const modelConfig: ModelConfig = {
          openEyes: false,
          closeEyes: false,
        };

        const { image, audio } = await loadThrowable(imageConfig, null);
        if (!image) {
          console.error("FAILED TO LOAD IMAGE");
          return;
        }
        for (let i = 0; i < 1; i += 1) {
          await throwItem(
            {
              imageConfig,
              throwConfig,
              soundConfig: null,
              faceConfig,
              modelConfig,
              modelParameters,
            },
            image,
            audio
          );
        }
      } catch (e) {
        console.error("failed to authorize", e);
      }
    }, 100);
  };

  socket.onerror = (event) => {
    console.error("socket error", event, socket);
  };

  // Set retry connect on close
  socket.onclose = () => {
    console.debug("Socket closed");

    clearInterval(retryInterval);
    retryInterval = setInterval(retryConnect, retryTimeout);

    clearRequestHandlers();
  };

  socket.onmessage = (event) => {
    if (!socket) return;
    handleSocketMessage(event);
  };

  return socket;
}

function retryConnect() {
  globalSocket = connect();
}

function handleSocketMessage(message: MessageEvent<string>) {
  const response: VTubeMessage<any> = JSON.parse(message.data);

  // Response didn't belong to any handler
  if (!response.requestID) {
    console.debug("request without identifier", response);
    return;
  }

  const requestID = Number(response.requestID);

  // We don't have a message handler available
  const handler = requestHandlers.get(requestID);
  if (handler === undefined) {
    console.debug("response for undefined handler", response);
    return;
  }

  requestHandlers.delete(requestID);

  if (response.messageType === "APIError") {
    // Handle API errors
    handler.reject(new APIError(response.data.errorID, response.data.message));
  } else {
    handler.resolve(response);
  }
}

export function sendSocketMessage(
  msg: VTubeMessage<any>
): Promise<VTubeMessage<any>> {
  return new Promise((resolve, reject) => {
    if (globalSocket.readyState !== WebSocket.OPEN) {
      throw new Error("socket not currently active");
    }

    // Update request ID
    let msgRequestID = requestID++;
    msg.requestID = "" + msgRequestID;

    const data = JSON.stringify(msg);

    requestHandlers.set(msgRequestID, { resolve, reject });
    globalSocket.send(data);
  });
}
