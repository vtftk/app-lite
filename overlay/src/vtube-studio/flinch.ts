const parametersH = ["FaceAngleX", "FaceAngleZ", "FacePositionX"];
const parametersV = ["FaceAngleY", "FacePositionY"];
const parametersE = ["EyeOpenLeft", "EyeOpenRight"];

type FlinchConfig = {
  multH: number;
  angle: number;
  mag: number;
  paramH: number[][];
  paramV: number[][];
  paramE: number[][];
  returnSpeed: number;
  eyeState: number;
};

export async function flinch(config: FlinchConfig) {}
