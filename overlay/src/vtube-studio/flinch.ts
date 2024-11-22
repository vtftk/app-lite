import { EyesMode } from "../vtftk/types";
import {
  injectParameterData,
  InjectParameterValue,
  ModelParameters,
} from "./model";
import { VTubeStudioWebSocket } from "./socket";

type FlinchConfig = {
  leftSide: boolean;
  angle: number;
  magnitude: number;
  modelParams: ModelParameters;
  returnSpeed: number;
  eyeState: EyesMode;
};

let flinchWeight = 0;
let flinchInterval: number | undefined;

export function flinch(socket: VTubeStudioWebSocket, config: FlinchConfig) {
  const parameterValues: InjectParameterValue[] = [];

  for (const param of config.modelParams.horizontal) {
    const value = config.leftSide ? param.max : param.min;

    parameterValues.push({
      id: param.name,
      value: value * config.magnitude,
    });
  }

  for (const param of config.modelParams.vertical) {
    let value = config.angle > 0 ? param.min : param.max;
    value = (value * Math.abs(config.angle)) / 45;

    parameterValues.push({
      id: param.name,
      value: value * config.magnitude,
    });
  }

  if (flinchInterval) clearInterval(flinchInterval);

  injectParameterData(socket, {
    faceFound: false,
    mode: "add",
    parameterValues,
  });

  flinchWeight = 1;
  flinchInterval = setInterval(flinchReturn, 1000.0 / 60.0, socket, config);
}

export function flinchReturn(
  socket: VTubeStudioWebSocket,
  config: FlinchConfig
) {
  flinchWeight -= 1 / config.returnSpeed / 60.0;
  if (flinchWeight <= 0) flinchWeight = 0;

  const parameterValues: InjectParameterValue[] = [];

  for (const param of config.modelParams.horizontal) {
    const value = config.leftSide ? param.max : param.min;
    parameterValues.push({
      id: param.name,
      value: value * config.magnitude * flinchWeight,
    });
  }

  for (const param of config.modelParams.vertical) {
    let value = config.angle > 0 ? param.min : param.max;
    value = (value * Math.abs(config.angle)) / 45;

    parameterValues.push({
      id: param.name,
      value: value * config.magnitude * flinchWeight,
    });
  }

  if (config.eyeState !== EyesMode.Unchanged) {
    for (const param of config.modelParams.eyes) {
      let value =
        (config.leftSide ? param.max : param.min) *
        config.magnitude *
        flinchWeight;

      if (config.eyeState === EyesMode.Closed) value = -value;

      parameterValues.push({
        id: param.name,
        value,
      });
    }
  }

  injectParameterData(socket, {
    faceFound: false,
    mode: "add",
    parameterValues,
  });

  if (flinchWeight == 0 && flinchInterval) clearInterval(flinchInterval);
}
