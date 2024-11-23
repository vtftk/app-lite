import { resolve } from "@tauri-apps/api/path";
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
  returnSpeed: number;
  eyeState: EyesMode;
};

let flinchWeight = 0;
let flinchInterval: number | undefined;

export function flinch(
  socket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: FlinchConfig
) {
  const parameterValues: InjectParameterValue[] = [];

  for (const param of modelParameters.horizontal) {
    const value = config.leftSide ? param.max : param.min;

    parameterValues.push({
      id: param.name,
      value: value * config.magnitude,
    });
  }

  for (const param of modelParameters.vertical) {
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
  flinchInterval = setInterval(
    flinchReturn,
    1000.0 / 60.0,
    socket,
    modelParameters,
    config
  );
}

function createFlinchReturn(
  socket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: FlinchConfig
) {
  let flinchWeight: number = 0;
  let flinchInterval: number | undefined;

  let _resolve: any;
  let _reject: any;

  // Cancel the flinch return
  const cancel = () => {
    if (flinchInterval) clearInterval(flinchInterval);
    if (_resolve) resolve();
  };

  const promise = new Promise(async (resolve, reject) => {
    _resolve = resolve;
    _reject = reject;

    flinchWeight = 1;
    flinchInterval = setInterval(
      flinchReturn,
      1000.0 / 60.0,
      socket,
      modelParameters,
      config
    );
  });

  return {
    promise,
    cancel,
  };
}

export function flinchReturn(
  socket: VTubeStudioWebSocket,
  modelParameters: ModelParameters,
  config: FlinchConfig
) {
  flinchWeight -= 1 / config.returnSpeed / 60.0;
  if (flinchWeight <= 0) flinchWeight = 0;

  const parameterValues: InjectParameterValue[] = [];

  for (const param of modelParameters.horizontal) {
    const value = config.leftSide ? param.max : param.min;
    parameterValues.push({
      id: param.name,
      value: value * config.magnitude * flinchWeight,
    });
  }

  for (const param of modelParameters.vertical) {
    let value = config.angle > 0 ? param.min : param.max;
    value = (value * Math.abs(config.angle)) / 45;

    parameterValues.push({
      id: param.name,
      value: value * config.magnitude * flinchWeight,
    });
  }

  if (config.eyeState !== EyesMode.Unchanged) {
    for (const param of modelParameters.eyes) {
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
