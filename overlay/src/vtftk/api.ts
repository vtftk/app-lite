import { BACKEND_HTTP } from "../constants";
import { CalibrationStepData } from "./calibration-types";
import { RuntimeAppData } from "./types";

export async function notifyProgressCalibration(body: CalibrationStepData) {
  await fetch(new URL("/calibration", BACKEND_HTTP), {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

export async function setRuntimeData(body: RuntimeAppData) {
  try {
    await fetch(new URL("/runtime-app-data", BACKEND_HTTP), {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    });
  } catch (e) {
    console.error("failed to set runtime data", e);
  }
}
