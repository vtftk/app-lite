import {
  ModelPosition,
  requestCurrentModel,
  requestMoveModel,
} from "../vtube-studio/model";
import {
  CalibrationPoint,
  CalibrationStep,
  CalibrationStepData,
} from "./calibration-types";
import { LARGEST_MODEL_SIZE, SMALLEST_MODEL_SIZE } from "../constants";

const calibrationEl = document.getElementById("calibration");

// Initial model position before calibration, to allow restoring back to
// original position after calibrating
let initialModelPosition: ModelPosition | undefined;

// Currently selected calibration point on the screen
let calibrationPoint: CalibrationPoint | undefined;

let currentStep: CalibrationStep = CalibrationStep.NotStarted;

let smallestPoint: CalibrationPoint | undefined;
let largestPoint: CalibrationPoint | undefined;

function setCalibrationPoint(x: number, y: number) {
  calibrationPoint = {
    x,
    y,
  };

  if (calibrationEl) {
    calibrationEl.style.left = `${x}px`;
    calibrationEl.style.top = `${y}px`;
  }
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

export async function notifyProgressCalibration(body: CalibrationStepData) {
  await fetch("http://localhost:58371/calibration", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

export async function beginCalibrationStep(step: CalibrationStep) {
  const currentStepIndex = STEPS.indexOf(currentStep);
  const stepIndex = STEPS.indexOf(step);

  console.log(stepIndex, currentStepIndex);

  // Handle out of sync with server
  if (
    stepIndex !== currentStepIndex + 1 ||
    step === CalibrationStep.NotStarted
  ) {
    await resetCalibration();
    return;
  }

  currentStep = step;

  switch (step) {
    // Capture initial model position
    case CalibrationStep.Smallest:
      const { modelPosition } = await requestCurrentModel();
      initialModelPosition = modelPosition;
      calibrationPoint = {
        x: window.innerWidth / 2,
        y: window.innerHeight / 2,
      };

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
        step: CalibrationStep.Complete,
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
    size: SMALLEST_MODEL_SIZE,
  });
}

function growModel() {
  return requestMoveModel({
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: 0,
    size: LARGEST_MODEL_SIZE,
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
