import { BACKEND_HTTP } from "../constants";
import { CalibrationStepData } from "./calibration-types";

export async function notifyProgressCalibration(body: CalibrationStepData) {
  await fetch(new URL("/calibration", BACKEND_HTTP), {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}
