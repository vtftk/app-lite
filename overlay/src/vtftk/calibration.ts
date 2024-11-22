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
import {
  BACKEND_HTTP,
  LARGEST_MODEL_SIZE,
  SMALLEST_MODEL_SIZE,
} from "../constants";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";

const calibrationEl = document.getElementById("calibration");

// Initial model position before calibration, to allow restoring back to
// original position after calibrating
let initialModelPosition: ModelPosition | undefined;
let modelId: string | undefined;

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

async function resetCalibration(socket: VTubeStudioWebSocket) {
  currentStep = CalibrationStep.NotStarted;
  await notifyProgressCalibration({ step: CalibrationStep.NotStarted });
  await resetModel(socket);
}

const STEPS = [
  CalibrationStep.NotStarted,
  CalibrationStep.Smallest,
  CalibrationStep.Largest,
  CalibrationStep.Complete,
];

export async function notifyProgressCalibration(body: CalibrationStepData) {
  await fetch(new URL("/calibration", BACKEND_HTTP), {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

export async function beginCalibrationStep(
  socket: VTubeStudioWebSocket,
  step: CalibrationStep
) {
  const currentStepIndex = STEPS.indexOf(currentStep);
  const stepIndex = STEPS.indexOf(step);

  console.log(stepIndex, currentStepIndex);

  // Handle out of sync with server
  if (
    stepIndex !== currentStepIndex + 1 ||
    step === CalibrationStep.NotStarted
  ) {
    await resetCalibration(socket);
    return;
  }

  currentStep = step;

  switch (step) {
    // Capture initial model position
    case CalibrationStep.Smallest:
      const { modelID, modelPosition } = await requestCurrentModel(socket);
      modelId = modelID;
      initialModelPosition = modelPosition;
      calibrationPoint = {
        x: window.innerWidth / 2,
        y: window.innerHeight / 2,
      };

      await shrinkModel(socket);
      await notifyProgressCalibration({ step: CalibrationStep.Smallest });
      break;

    // Store smallest position and grow model
    case CalibrationStep.Largest:
      smallestPoint = await getModelGuidePosition(socket);
      await notifyProgressCalibration({ step: CalibrationStep.Largest });
      await growModel(socket);
      break;

    // Store largest position, report calibration results and reset model
    case CalibrationStep.Complete:
      if (smallestPoint === undefined || modelId === undefined) {
        await resetCalibration(socket);
        console.error("missing smallest point");
        return;
      }

      largestPoint = await getModelGuidePosition(socket);
      await notifyProgressCalibration({
        step: CalibrationStep.Complete,
        model_id: modelId,
        smallest_point: smallestPoint,
        largest_point: largestPoint,
      });
      await resetModel(socket);
      break;
    default:
      break;
  }
}

function shrinkModel(socket: VTubeStudioWebSocket) {
  return requestMoveModel(socket, {
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: 0,
    size: SMALLEST_MODEL_SIZE,
  });
}

function growModel(socket: VTubeStudioWebSocket) {
  return requestMoveModel(socket, {
    timeInSeconds: 0.5,
    valuesAreRelativeToModel: false,
    rotation: 0,
    size: LARGEST_MODEL_SIZE,
  });
}

function resetModel(socket: VTubeStudioWebSocket) {
  if (!initialModelPosition) return Promise.resolve({});

  return requestMoveModel(socket, {
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
async function getModelGuidePosition(
  socket: VTubeStudioWebSocket
): Promise<CalibrationPoint> {
  if (calibrationPoint === undefined)
    throw new Error("calibration not currently active");

  // Calibration, get initial position
  const { modelPosition } = await requestCurrentModel(socket);

  return {
    x:
      modelPosition.positionX -
      ((calibrationPoint.x / window.innerWidth) * 2 - 1),
    y:
      modelPosition.positionY +
      ((calibrationPoint.y / window.innerHeight) * 2 - 1),
  };
}
