import { BACKEND_HTTP } from "../constants";
import { CalibrationStepData } from "./calibration-types";
import { ModelData, RuntimeAppData } from "./types";

export async function notifyProgressCalibration(body: CalibrationStepData) {
  const response = await fetch(new URL("/calibration", BACKEND_HTTP), {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });

  return response.json();
}

export async function getCalibrationData(): Promise<ModelData[]> {
  const response = await fetch(new URL("/calibration-data", BACKEND_HTTP));
  return response.json();
}

export async function updateRuntimeData(body: Partial<RuntimeAppData>) {
  try {
    await fetch(new URL("/runtime-app-data", BACKEND_HTTP), {
      method: "PUT",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    });
  } catch (e) {
    console.error("failed to set runtime data", e);
  }
}
