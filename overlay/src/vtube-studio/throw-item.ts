import { percentRange, randomBool, randomRange } from "../math";
import { InputParameter, ModelPosition, requestCurrentModel } from "./model";

export type ThrowItemConfig = {
  imageConfig: ImageConfig;
  throwConfig: ThrowConfig;
  soundConfig: SoundConfig | null;
  faceConfig: FaceConfig;
  modelConfig: ModelConfig;
};

export enum ThrowDirection {
  Random = "Random",
  LeftOnly = "Left Only",
  RightOnly = "Right Only",
}

export type ImageConfig = {
  // Image being thrown
  src: string;
  // Weight of thrown item
  weight: number;
  // Scale of thrown item
  scale: number;
  // Render pixelate
  pixel: boolean;
};

export type FaceConfig = {
  /// Minimum and maximum x position of the model face
  x: MinMax;
  /// Minimum and maximum y position of the model face
  y: MinMax;
};

export type SoundConfig = {
  // URL of the sound to play
  src: string;

  // Volume to play sound at
  volume: number;
};

export type ModelConfig = {
  // Whether to close model eyes when hit
  closeEyes: boolean;
  // Whether to open model eyes when hit
  openEyes: boolean;
};

export type TimingConfig = {};

type MinMax = {
  min: number;
  max: number;
};

export type ThrowConfig = {
  // Direction thrown from
  direction: ThrowDirection;
  throwAngle: MinMax;
  itemScale: MinMax;
  spinSpeed: MinMax;

  // Duration of the whole throw animation
  duration: number;

  // Delay before the item is thrown
  delay: number;
};

function isRandomDirectionLeft(direction: ThrowDirection): boolean {
  switch (direction) {
    case ThrowDirection.Random:
      return Math.random() < 0.5;
    case ThrowDirection.LeftOnly:
      return true;
    case ThrowDirection.RightOnly:
      return false;
    default:
      return false;
  }
}

export async function throwItem(
  config: ThrowItemConfig,
  image: HTMLImageElement,
  audio: HTMLAudioElement | null
) {
  const { modelPosition } = await requestCurrentModel();

  // Model is not available
  if (!modelPosition) return;

  const { imageConfig, faceConfig, throwConfig, modelConfig, soundConfig } =
    config;

  const modelScale = (modelPosition.size + 100) / 200;

  const offsetX = percentRange(modelScale, faceConfig.x.min, faceConfig.x.max);
  const offsetY = percentRange(modelScale, faceConfig.y.min, faceConfig.y.max);

  console.log(offsetX);

  const xPos = (modelPosition.positionX - offsetX + 1) / 2;
  const yPos = 1 - (modelPosition.positionY - offsetY + 1) / 2;

  const isLeft: boolean = isRandomDirectionLeft(throwConfig.direction);

  // Multiplier on the x axis
  const xMulti = isLeft ? 1 : -1;

  const angle =
    randomRange(throwConfig.throwAngle.min, throwConfig.throwAngle.max) *
    xMulti;

  const sizeScale =
    throwConfig.itemScale.min +
    modelScale * (throwConfig.itemScale.max - throwConfig.itemScale.min);

  const eyeState = modelConfig.closeEyes ? 1 : modelConfig.openEyes ? 2 : 0;

  const randScale = (modelPosition.size + 100) / 200;
  const randH = (Math.random() * 100 - 50) * randScale;
  const randV = (Math.random() * 100 - 50) * randScale;

  const scaledImageWidth = image.width * imageConfig.scale * sizeScale;
  const scaledImageHeight = image.height * imageConfig.scale * sizeScale;

  const thrown = createThrownImage(
    imageConfig,
    image,
    scaledImageWidth,
    scaledImageHeight,
    angle,
    throwConfig.spinSpeed
  );

  const movement = createMovementContainer(
    isLeft,
    throwConfig.duration,
    throwConfig.delay
  );

  const pivot = createPivotContainer(
    scaledImageWidth,
    scaledImageHeight,
    xPos,
    yPos,
    randH,
    randV,
    angle
  );

  const root = createRootContainer(modelPosition);

  movement.appendChild(thrown);
  pivot.appendChild(movement);
  root.appendChild(pivot);
  document.body.appendChild(root);

  console.log("THROW");
}

function createRootContainer(modelPosition: ModelPosition) {
  const elm = document.createElement("div");
  elm.classList.add("thrown");

  const style = elm.style;

  const originXPercent = ((modelPosition.positionX + 1) / 2) * 100;
  const originYPercent = (1 - (modelPosition.positionY + 1) / 2) * 100;

  style.width = "100%";
  style.height = "100%";
  style.transformOrigin = `${originXPercent}% ${originYPercent}%`;

  return elm;
}

function createThrownImage(
  config: ImageConfig,
  image: HTMLImageElement,
  scaledWidth: number,
  scaledHeight: number,

  angle: number,
  spinSpeed: MinMax
): HTMLImageElement {
  const elm = document.createElement("img");
  elm.src = image.src;
  elm.classList.add("animated");
  const style = elm.style;

  style.width = `${scaledWidth}px`;
  style.height = `${scaledHeight}px`;
  style.imageRendering = config.pixel ? "pixelated" : "auto";

  // Spin speed is zero, should immediately spin all the way
  if (spinSpeed.max - spinSpeed.min === 0) {
    style.transform = "rotate(" + -angle + "deg)";
    return elm;
  }

  const clockwise = randomBool();
  const animationDuration = 3 / randomRange(spinSpeed.min, spinSpeed.max);

  style.animationName = clockwise ? "spinClockwise" : "spinCounterClockwise";
  style.animationDuration = `${animationDuration}s`;
  style.animationIterationCount = "infinite";

  // TODO: SLOW DOWN NEAR END? 1  / randomRange(spinSpeed.min, spinSpeed.max); AFTER data.throwDuration * 500 + data.delay

  return elm;
}

function createPivotContainer(
  scaledWidth: number,
  scaledHeight: number,
  xPos: number,
  yPos: number,
  randH: number,
  randV: number,
  angle: number
) {
  const elm = document.createElement("div");
  elm.classList.add("thrown");

  const style = elm.style;

  const left = window.innerWidth * xPos - scaledWidth / 2 + randH;
  const top = window.innerHeight * yPos - scaledHeight / 2 + randV;

  style.left = `${left}px`;
  style.top = `${top}px`;
  style.transform = "rotate(" + angle + "deg)";

  return elm;
}

function createMovementContainer(
  leftSide: boolean,
  duration: number,
  delayMs: number
) {
  const elm = document.createElement("div");
  elm.classList.add("animated");

  const style = elm.style;

  style.animationName = leftSide ? "throwLeft" : "throwRight";
  style.animationDuration = `${duration}ms`;
  style.animationDelay = `${delayMs}ms`;

  return elm;
}

/**
 * Loads the resources a throwable depends on (Image and optionally audio)
 *
 * @param imageConfig The image configuration
 * @param soundConfig The sound configuration
 * @returns
 */
export async function loadThrowable(
  imageConfig: ImageConfig,
  soundConfig: SoundConfig | null
): Promise<{ image: HTMLImageElement | null; audio: HTMLAudioElement | null }> {
  // Load the image and audio if present
  const [imageResult, audioResult] = await Promise.allSettled([
    // Load the image
    loadImage(imageConfig.src),

    // Load the sound
    soundConfig ? loadAudio(soundConfig.src) : Promise.reject(),
  ]);

  let image: HTMLImageElement | null =
    imageResult.status === "fulfilled" ? imageResult.value : null;
  let audio: HTMLAudioElement | null =
    audioResult.status === "fulfilled" ? audioResult.value : null;

  return { image, audio };
}

async function loadImage(src: string): Promise<HTMLImageElement> {
  const image = new Image();
  image.src = src;

  return new Promise((resolve, reject) => {
    image.onload = () => resolve(image);
    image.onerror = (err) => reject(err);
  });
}

async function loadAudio(src: string): Promise<HTMLAudioElement> {
  const audio = new Audio(src);

  return new Promise((resolve, reject) => {
    audio.oncanplaythrough = () => resolve(audio);
    audio.onerror = () => reject();
  });
}

type ModelParameters = {
  horizontal: [ModelParameter, ModelParameter, ModelParameter];
  vertical: [ModelParameter, ModelParameter];
  eyes: [ModelParameter, ModelParameter];
};

type ModelParameter = {
  name: string;
  value: number;
  min: number;
  max: number;
};

// Names for horizontal input parameters
const HORIZONTAL_PARAM_NAMES = ["FaceAngleX", "FaceAngleZ", "FacePositionX"];
// Names for vertical input parameters
const VERTICAL_PARAM_NAMES = ["FaceAngleY", "FacePositionY"];
// Names for eye open parameters
const EYE_PARAM_NAMES = ["EyeOpenLeft", "EyeOpenRight"];

export function createModelParameters(
  params: InputParameter[]
): ModelParameters {
  const getOrDefault = (
    name: string,
    value: number,
    min: number,
    max: number
  ): ModelParameter => {
    const param = params.find((value) => value.name === name);
    if (param) {
      return { name, value: param.value, min: param.min, max: param.max };
    }

    return { name, value, min, max };
  };

  const horizontal = HORIZONTAL_PARAM_NAMES.map((name) =>
    getOrDefault(name, 0, -30, 30)
  ) as [ModelParameter, ModelParameter, ModelParameter];

  const vertical = VERTICAL_PARAM_NAMES.map((name) =>
    getOrDefault(name, 0, -30, 30)
  ) as [ModelParameter, ModelParameter];

  const eyes = EYE_PARAM_NAMES.map((name) => getOrDefault(name, 0, 0, 1)) as [
    ModelParameter,
    ModelParameter
  ];

  return { horizontal, vertical, eyes };
}
