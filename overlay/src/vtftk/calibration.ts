import { event } from "@tauri-apps/api";
import {
  ModelPosition,
  requestCurrentModel,
  requestMoveModel,
} from "../vtube-studio/model";
import { notifyProgressCalibration } from "./events";
import { CalibrationPoint, CalibrationStep } from "./calibration-types";

// Initial model position before calibration, to allow restoring back to
// original position after calibrating
let initialModelPosition: ModelPosition | undefined;

// Currently selected calibration point on the screen
let calibrationPoint: CalibrationPoint | undefined;

let currentStep: CalibrationStep = CalibrationStep.NotStarted;

let defaultPoint: CalibrationPoint | undefined;
let smallestPoint: CalibrationPoint | undefined;
let largestPoint: CalibrationPoint | undefined;

function setCalibrationPoint(x: number, y: number) {
  calibrationPoint = {
    x,
    y,
  };
}

window.addEventListener("click", (event) => {
  setCalibrationPoint(event.clientX, event.clientY);
});

async function resetCalibration() {
  currentStep = CalibrationStep.NotStarted;
  await notifyProgressCalibration({ step: CalibrationStep.NotStarted });
  await resetModel();
}

const STEPS = [
  CalibrationStep.NotStarted,
  CalibrationStep.Smallest,
  CalibrationStep.Largest,
  CalibrationStep.Complete,
];

async function beginCalibrationStep(step: CalibrationStep) {
  const currentStepIndex = STEPS.indexOf(currentStep);
  const stepIndex = STEPS.indexOf(step);

  // Handle out of sync with server
  if (
    stepIndex !== currentStepIndex + 1 ||
    step === CalibrationStep.NotStarted
  ) {
    await resetCalibration();
    return;
  }

  switch (step) {
    // Capture initial model position
    case CalibrationStep.Smallest:
      const { modelPosition } = await requestCurrentModel();
      initialModelPosition = modelPosition;

      await shrinkModel();
      await notifyProgressCalibration({ step: CalibrationStep.Smallest });
      break;

    // Store smallest position and grow model
    case CalibrationStep.Largest:
      smallestPoint = await getModelGuidePosition();
      await notifyProgressCalibration({ step: CalibrationStep.Largest });
      await growModel();
      break;

    // Store largest position, report calibration results and reset model
    case CalibrationStep.Complete:
      if (smallestPoint === undefined) {
        await resetCalibration();
        console.error("missing smallest point");
        return;
      }

      largestPoint = await getModelGuidePosition();
      await notifyProgressCalibration({
        step: currentStep,
        smallest_point: smallestPoint,
        largest_point: largestPoint,
      });
      await resetModel();
      break;
    default:
      break;
  }
}

function shrinkModel() {
  return requestMoveModel({
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: 0,
    size: -100,
  });
}

function growModel() {
  return requestMoveModel({
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: 0,
    size: 100,
  });
}

function resetModel() {
  if (!initialModelPosition) return Promise.resolve({});

  return requestMoveModel({
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: initialModelPosition.rotation,
    size: initialModelPosition.size,
    positionX: initialModelPosition.positionX,
    positionY: initialModelPosition.positionY,
  });
}

/**
 * Obtains the current model position relative
 * to the guide
 */
async function getModelGuidePosition(): Promise<CalibrationPoint> {
  if (calibrationPoint === undefined)
    throw new Error("calibration not currently active");

  // Calibration, get initial position
  const { modelPosition } = await requestCurrentModel();

  return {
    x:
      modelPosition.positionX -
      ((calibrationPoint.x / window.innerWidth) * 2 - 1),
    y:
      modelPosition.positionY +
      ((calibrationPoint.y / window.innerHeight) * 2 - 1),
  };
}
