import { InvalidMessageTypeError } from "./error";
import { createVTubeMessage } from "./message";
import { sendSocketMessage } from "./socket";

type RequestCurrentModelData = {
  hasPhysicsFile: boolean;
  live2DModelName: string;
  modelID: string;
  modelLoadTime: number;
  modelLoaded: boolean;
  modelName: string;
  modelPosition: {
    positionX: number;
    positionY: number;
    rotation: number;
    size: number;
  };
  numberOfLive2DArtmeshes: number;
  numberOfLive2DParameters: number;
  numberOfTextures: number;
  textureResolution: number;
  timeSinceModelLoaded: number;
  vtsModelIconName: string;
  vtsModelName: string;
};

export async function requestCurrentModel() {
  const request = createVTubeMessage("CurrentModelRequest", undefined);
  const response = await sendSocketMessage(request);

  if (response.messageType !== "CurrentModelResponse") {
    throw new InvalidMessageTypeError(
      "CurrentModelResponse",
      response.messageType
    );
  }

  const data: RequestCurrentModelData = response.data;
  return data;
}
