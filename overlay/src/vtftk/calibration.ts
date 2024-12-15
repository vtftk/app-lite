import { ModelData } from "$shared/appData";

import { notifyProgressCalibration } from "./api";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { LARGEST_MODEL_SIZE, SMALLEST_MODEL_SIZE } from "../constants";
import { CalibrationStep, CalibrationPoint } from "./calibration-types";
import {
  ModelPosition,
  requestMoveModel,
  requestCurrentModel,
} from "../vtube-studio/model";

const calibrationEl = document.getElementById("calibration");

// Initial model position before calibration, to allow restoring back to
// original position after calibrating
let initialModelPosition: ModelPosition | undefined;
let modelId: string | undefined;
let modelName: string | undefined;

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

let dragging = true;

function onMouseDown(event: MouseEvent) {
  setCalibrationPoint(event.clientX, event.clientY);
  dragging = true;
}

function onMouseUp() {
  dragging = false;
}

function onMouseMove(event: MouseEvent) {
  if (dragging) setCalibrationPoint(event.clientX, event.clientY);
}

function subscribeCalibrate() {
  if (calibrationEl) {
    calibrationEl.style.visibility = "visible";
    calibrationEl.hidden = false;
  }

  setCalibrationPoint(window.innerWidth / 2, window.innerHeight / 2);

  window.addEventListener("mousedown", onMouseDown);
  window.addEventListener("mouseup", onMouseUp);
  window.addEventListener("mousemove", onMouseMove);
}

function unsubscribeCalibrate() {
  if (calibrationEl) {
    calibrationEl.style.visibility = "hidden";
    calibrationEl.hidden = true;
  }
  window.removeEventListener("mousedown", onMouseDown);
  window.removeEventListener("mouseup", onMouseUp);
  window.removeEventListener("mousemove", onMouseMove);
}

async function resetCalibration(
  socket: VTubeStudioWebSocket,
  resetStep = true,
) {
  if (resetStep) {
    currentStep = CalibrationStep.NotStarted;
  }

  modelId = undefined;
  modelName = undefined;

  unsubscribeCalibrate();

  if (resetStep) {
    await notifyProgressCalibration({ step: CalibrationStep.NotStarted });
  }

  await resetModel(socket);
}

const STEPS = [
  CalibrationStep.NotStarted,
  CalibrationStep.Smallest,
  CalibrationStep.Largest,
  CalibrationStep.Complete,
];

export async function beginCalibrationStep(
  socket: VTubeStudioWebSocket,
  step: CalibrationStep,
  onCalibrationComplete: (data: ModelData) => void,
) {
  const currentStepIndex = STEPS.indexOf(currentStep);
  const stepIndex = STEPS.indexOf(step);

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
    case CalibrationStep.Smallest: {
      subscribeCalibrate();

      const {
        modelID,
        modelName: _modelName,
        modelPosition,
      } = await requestCurrentModel(socket);

      dragging = true;
      modelId = modelID;
      modelName = _modelName;
      initialModelPosition = modelPosition;
      calibrationPoint = {
        x: window.innerWidth / 2,
        y: window.innerHeight / 2,
      };

      await shrinkModel(socket);
      await notifyProgressCalibration({ step: CalibrationStep.Smallest });
      break;
    }

    // Store smallest position and grow model
    case CalibrationStep.Largest: {
      dragging = true;
      smallestPoint = await getModelGuidePosition(socket);
      await notifyProgressCalibration({ step: CalibrationStep.Largest });
      await growModel(socket);
      break;
    }

    // Store largest position, report calibration results and reset model
    case CalibrationStep.Complete: {
      if (
        smallestPoint === undefined ||
        modelId === undefined ||
        modelName === undefined
      ) {
        await resetCalibration(socket);
        console.error("calibration was not started");
        return;
      }

      largestPoint = await getModelGuidePosition(socket);
      const { model_data } = await notifyProgressCalibration({
        step: CalibrationStep.Complete,
        model_id: modelId,
        model_name: modelName,
        smallest_point: smallestPoint,
        largest_point: largestPoint,
      });
      await resetCalibration(socket, false);
      onCalibrationComplete(model_data);
      break;
    }
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
  socket: VTubeStudioWebSocket,
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
