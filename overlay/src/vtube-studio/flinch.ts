import {
  injectParameterData,
  InjectParameterValue,
  ModelParameters,
} from "./model";

type FlinchConfig = {
  leftSide: boolean;
  angle: number;
  magnitude: number;
  modelParams: ModelParameters;
  returnSpeed: number;
  eyeState: number;
};

export async function flinch(config: FlinchConfig) {
  const parameterValues: InjectParameterValue[] = [];

  for (const param of config.modelParams.horizontal) {
    parameterValues.push({
      id: param.name,
      value: (config.leftSide ? param.max : param.min) * config.magnitude,
    });
  }

  for (const param of config.modelParams.vertical) {
    const value =
      (((config.angle > 0 ? param.min : param.max) * Math.abs(config.angle)) /
        45) *
      config.magnitude;
    parameterValues.push({
      id: param.name,
      value,
    });
  }

  await injectParameterData({
    faceFound: false,
    mode: "add",
    parameterValues,
  });
}
