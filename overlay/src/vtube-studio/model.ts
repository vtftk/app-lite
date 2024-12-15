import { createVTubeMessage } from "./message";
import { VTubeStudioWebSocket } from "./socket";
import { InvalidMessageTypeError } from "./error";

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

export async function requestCurrentModel(socket: VTubeStudioWebSocket) {
  const request = createVTubeMessage("CurrentModelRequest", undefined);
  const response = await socket.send(request);

  if (response.messageType !== "CurrentModelResponse") {
    throw new InvalidMessageTypeError(
      "CurrentModelResponse",
      response.messageType,
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

export async function requestInputParameterList(
  socket: VTubeStudioWebSocket,
): Promise<InputParameterListData> {
  const request = createVTubeMessage("InputParameterListRequest", undefined);
  const response = await socket.send(request);

  if (response.messageType !== "InputParameterListResponse") {
    throw new InvalidMessageTypeError(
      "InputParameterListResponse",
      response.messageType,
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

export async function requestMoveModel(
  socket: VTubeStudioWebSocket,
  data: MoveModelData,
) {
  const request = createVTubeMessage("MoveModelRequest", data);
  const response = await socket.send(request);
  return response.data;
}

export type InjectParameterData = {
  faceFound: boolean;
  mode: string;
  parameterValues: InjectParameterValue[];
};

export type InjectParameterValue = {
  id: string;
  value: number;
};

export async function injectParameterData(
  socket: VTubeStudioWebSocket,
  data: InjectParameterData,
) {
  const request = createVTubeMessage("InjectParameterDataRequest", data);
  const response = await socket.send(request);
  return response.data;
}

export type ModelParameters = {
  horizontal: [ModelParameter, ModelParameter, ModelParameter];
  vertical: [ModelParameter, ModelParameter];
  eyes: [ModelParameter, ModelParameter];
};

export type ModelParameter = {
  name: string;
  value: number;
  min: number;
  max: number;
};

// Names for horizontal input parameters
const HORIZONTAL_PARAM_NAMES = ["FaceAngleX", "FaceAngleZ", "FacePositionX"];
// Names for vertical input parameters
const VERTICAL_PARAM_NAMES = ["FaceAngleY", "FacePositionY"];
// Names for eye open parameters
const EYE_PARAM_NAMES = ["EyeOpenLeft", "EyeOpenRight"];

export function createModelParameters(
  params: InputParameter[],
): ModelParameters {
  const getOrDefault = (
    name: string,
    value: number,
    min: number,
    max: number,
  ): ModelParameter => {
    const param = params.find((value) => value.name === name);
    if (param) {
      return { name, value: param.value, min: param.min, max: param.max };
    }

    return { name, value, min, max };
  };

  const horizontal = HORIZONTAL_PARAM_NAMES.map((name) =>
    getOrDefault(name, 0, -30, 30),
  ) as [ModelParameter, ModelParameter, ModelParameter];

  const vertical = VERTICAL_PARAM_NAMES.map((name) =>
    getOrDefault(name, 0, -30, 30),
  ) as [ModelParameter, ModelParameter];

  const eyes = EYE_PARAM_NAMES.map((name) => getOrDefault(name, 0, 0, 1)) as [
    ModelParameter,
    ModelParameter,
  ];

  return { horizontal, vertical, eyes };
}
