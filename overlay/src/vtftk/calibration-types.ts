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

export type CalibrationStepData =
  | { step: CalibrationStep.NotStarted }
  | { step: CalibrationStep.Smallest }
  | { step: CalibrationStep.Largest }
  | {
      step: CalibrationStep.Complete;
      largest_point: CalibrationPoint;
      smallest_point: CalibrationPoint;
    };

export type CalibrationPoint = { x: number; y: number };
