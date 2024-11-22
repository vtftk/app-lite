import { InvalidMessageTypeError } from "./error";
import { createVTubeMessage, VTubeMessage } from "./message";
import { sendSocketMessage } from "./socket";

export type RequestCurrentModelData = {
  hasPhysicsFile: boolean;
  live2DModelName: string;
  modelID: string;
  modelLoadTime: number;
  modelLoaded: boolean;
  modelName: string;
  modelPosition: ModelPosition;
  numberOfLive2DArtmeshes: number;
  numberOfLive2DParameters: number;
  numberOfTextures: number;
  textureResolution: number;
  timeSinceModelLoaded: number;
  vtsModelIconName: string;
  vtsModelName: string;
};

export type ModelPosition = {
  positionX: number;
  positionY: number;
  rotation: number;
  size: number;
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

export type InputParameter = {
  name: string;
  addedBy: string;
  value: number;
  min: number;
  max: number;
  defaultValue: number;
};

export type InputParameterListData = {
  modelLoaded: boolean;
  modelName: string;
  modelID: string;
  defaultParameters: InputParameter[];
  customParameters: [];
};

export async function requestInputParameterList(): Promise<InputParameterListData> {
  const request = createVTubeMessage("InputParameterListRequest", undefined);
  const response = await sendSocketMessage(request);

  if (response.messageType !== "InputParameterListResponse") {
    throw new InvalidMessageTypeError(
      "InputParameterListResponse",
      response.messageType
    );
  }

  return response.data;
}

type MoveModelData = {
  timeInSeconds: number;
  valuesAreRelativeToModel: boolean;
  rotation?: number;
  size?: number;
  positionX?: number;
  positionY?: number;
};

export async function requestMoveModel(data: MoveModelData) {
  const request = createVTubeMessage("MoveModelRequest", data);
  const response = await sendSocketMessage(request);
  console.log("Move model response", response);
  return response.data;
}
