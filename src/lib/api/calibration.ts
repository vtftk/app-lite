import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

export enum CalibrationStep {
  NotStarted = "NotStarted",

  // Original model position is known, we have shrunk to the smallest
  // size preparing to capture the min face X Y
  Smallest = "Smallest",

  // Smallest model position is known, we have grown to the largest
  // size preparing to capture the max face X Y
  Largest = "Largest",

  // Both positions are known
  Complete = "Complete",
}

export const calibrationState = writable(CalibrationStep.NotStarted);

// Handle logout
listen<{ step: CalibrationStep }>(
  "calibration_state",
  ({ payload: { step } }) => {
    calibrationState.set(step);
    console.log(step);
  }
);
