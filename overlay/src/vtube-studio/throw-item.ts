import { sleep } from "../async-utils";
import { LARGEST_MODEL_SIZE, TOTAL_MODEL_SIZE_RANGE } from "../constants";
import { percentRange, randomBool, randomRange } from "../math";
import { AppData, MinMax, ModelData, ThrowDirection } from "../vtftk/config";
import { flinch } from "./flinch";
import { ModelParameters, ModelPosition, requestCurrentModel } from "./model";

export type ThrowItemConfig = {
  imageConfig: ImageConfig;
  soundConfig: SoundConfig | null;
  modelParameters: ModelParameters;
};

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

export type SoundConfig = {
  // URL of the sound to play
  src: string;

  // Volume to play sound at
  volume: number;
};

function isRandomDirectionLeft(direction: ThrowDirection): boolean {
  switch (direction) {
    case ThrowDirection.Random:
      return randomBool();
    case ThrowDirection.LeftOnly:
      return true;
    case ThrowDirection.RightOnly:
      return false;
    default:
      return false;
  }
}

/**
 * Throws an item
 *
 * @param appData App data
 * @param config Configuration for the thrown item
 * @param image Preloaded image for the item
 * @param audio Preloaded audio for the item
 * @returns Promise that completes when the item has been thrown and removed
 */
export async function throwItem(
  appData: AppData,
  config: ThrowItemConfig,
  image: HTMLImageElement,
  audio: HTMLAudioElement | null
) {
  const { modelID, modelPosition } = await requestCurrentModel();

  const modelData = appData.models[modelID];

  // Model is not yet calibrated
  if (modelData === undefined) return;

  // Model is not available
  if (!modelPosition) return;

  const { throwables, items } = appData;
  const { imageConfig } = config;

  // Determine scale of the model relative to the calibrated minimum and maximum sizes
  const modelScale =
    (modelPosition.size + LARGEST_MODEL_SIZE) / TOTAL_MODEL_SIZE_RANGE;

  const leftSide: boolean = isRandomDirectionLeft(throwables.direction);

  let angle = randomRange(
    throwables.throw_angle.min,
    throwables.throw_angle.max
  );

  // Flip the angle when coming from the right side
  if (!leftSide) angle = -angle;

  const itemScale = percentRange(
    modelScale,
    items.item_scale.min,
    items.item_scale.max
  );

  const scaledImageWidth = image.width * imageConfig.scale * itemScale;
  const scaledImageHeight = image.height * imageConfig.scale * itemScale;

  const thrown = createThrownImage(
    imageConfig,
    image,
    scaledImageWidth,
    scaledImageHeight,
    angle,
    throwables.spin_speed
  );

  const movement = createMovementContainer(
    leftSide,
    throwables.duration,
    throwables.impact_delay
  );

  const pivot = createPivotContainer(
    scaledImageWidth,
    scaledImageHeight,
    modelPosition,
    modelData,
    modelScale,
    angle
  );

  const root = createRootContainer(modelPosition);

  movement.appendChild(thrown);
  pivot.appendChild(movement);
  root.appendChild(pivot);
  document.body.appendChild(root);

  // Impact is encountered half way through the animation
  const impactTimeout = throwables.duration / 2 + throwables.impact_delay;

  // Wait for the impact to occur
  await sleep(impactTimeout);

  // Handle point of impact
  handleThrowableImpact(appData, config, audio, angle, leftSide);

  // Wait remaining duration before removing
  await sleep(throwables.duration / 2);

  // Remove after complete
  document.body.removeChild(root);
}

async function handleThrowableImpact(
  appData: AppData,
  config: ThrowItemConfig,
  audio: HTMLAudioElement | null,
  angle: number,
  leftSide: boolean
) {
  // Play the impact sound
  if (audio !== null && config.soundConfig) {
    try {
      audio.volume = appData.items.global_volume * config.soundConfig.volume;

      audio.play();
    } catch (err) {
      console.error("failed to play audio", err);
    }
  }

  // Make the VTuber model flinch from the impact
  flinch({
    angle,
    eyeState: appData.model.eyes_on_hit,
    magnitude: config.imageConfig.weight,
    modelParams: config.modelParameters,
    leftSide,
    returnSpeed: 0.3,
  });

  // TODO: IMPACT DECAL
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

/**
 * Creates a throwable image element, applies rotation and other
 * image styling
 *
 * @param config Configuration for the image
 * @param image The underlying image element itself
 * @param scaledWidth The scaled width of the image
 * @param scaledHeight The scaled height of the image
 * @param angle The angle of the image
 * @param spinSpeed Speed the image should spin at
 * @returns The image element
 */
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

/**
 * Creates a container for handling the pivoting of a throwable
 *
 * @param scaledWidth Scaled width of the image
 * @param scaledHeight Scaled height of the image
 * @param modelPosition The position of the model
 * @param modelData Model and calibration data for the model
 * @param modelScale Scale of the model
 * @param angle Angle of the throwable
 * @returns The container element
 */
function createPivotContainer(
  scaledWidth: number,
  scaledHeight: number,
  modelPosition: ModelPosition,
  modelData: ModelData,
  modelScale: number,
  angle: number
) {
  const elm = document.createElement("div");
  elm.classList.add("thrown");

  const style = elm.style;

  const offsetX = percentRange(modelScale, modelData.x.min, modelData.x.max);
  const offsetY = percentRange(modelScale, modelData.y.min, modelData.y.max);

  const xPos = (modelPosition.positionX - offsetX + 1) / 2;
  const yPos = 1 - (modelPosition.positionY - offsetY + 1) / 2;

  // Random offsets to the X and Y positions
  const randX = (Math.random() * 100 - 50) * modelScale;
  const randY = (Math.random() * 100 - 50) * modelScale;

  const left = window.innerWidth * xPos - scaledWidth / 2 + randX;
  const top = window.innerHeight * yPos - scaledHeight / 2 + randY;

  style.left = `${left}px`;
  style.top = `${top}px`;
  style.transform = "rotate(" + angle + "deg)";

  return elm;
}

/**
 * Creates the container in charge of the movement for
 * a throwable item
 *
 * @param leftSide Whether the movement is coming from the left side
 * @param duration The duration of the whole animation
 * @param delayMs Delay before the movement begins
 * @returns The container element
 */
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
export async function loadThrowableResources(
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
