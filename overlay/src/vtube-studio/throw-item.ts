import { randomBool, rangedRandom } from "../math";
import { requestCurrentModel } from "./model";

type ThrowItemConfig = {
  imageConfig: ImageConfig;
  throwConfig: ThrowConfig;
  soundConfig: SoundConfig | null;
  faceConfig: FaceConfig;
  physicsConfig: PhysicsConfig;
  modelConfig: ModelConfig;
};

enum ThrowDirection {
  Random = "Random",
  LeftOnly = "Left Only",
  RightOnly = "Right Only",
}

type ImageConfig = {
  // Image being thrown
  src: string;
  // Weight of thrown item
  weight: number;
  // Scale of thrown item
  scale: number;
  // Render pixelate
  pixel: boolean;
};

type FaceConfig = {
  widthMin: number;
  widthMax: number;

  heightMin: number;
  heightMax: number;
};

type SoundConfig = {
  // URL of the sound to play
  src: string;

  // Volume to play sound at
  volume: number;
};

type PhysicsConfig = {
  physicsSim: boolean;
  physicsFPS: number;
  physicsHorizontal: number;
  physicsVertical: number;
  physicsGravity: number;
  physicsReverse: boolean;
};

type ModelConfig = {
  // Whether to close model eyes when hit
  closeEyes: boolean;
  // Whether to open model eyes when hit
  openEyes: boolean;
};

type TimingConfig = {};

type MinMax = {
  min: number;
  max: number;
};

type ThrowConfig = {
  // Direction thrown from
  direction: ThrowDirection;
  throwAngle: MinMax;
  itemScale: MinMax;
  spinSpeed: MinMax;

  // Duration of the whole throw animation
  duration: number;

  // Delay before the item is thrown
  delay: number;

  // Adjustment to the volume
  volume: number;
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

async function throwItem(
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

  const offsetX =
    faceConfig.widthMin +
    modelScale * (faceConfig.widthMax - faceConfig.widthMin);
  const offsetY =
    faceConfig.heightMin +
    modelScale * (faceConfig.heightMax - faceConfig.heightMin);

  const xPos = (modelPosition.positionX - offsetX + 1) / 2;
  const yPos = 1 - (modelPosition.positionY - offsetY + 1) / 2;

  const isLeft: boolean = isRandomDirectionLeft(throwConfig.direction);

  // Multiplier on the x axis
  const xMulti = isLeft ? 1 : -1;

  const angle =
    rangedRandom(throwConfig.throwAngle.min, throwConfig.throwAngle.max) *
    xMulti;

  const sizeScale =
    throwConfig.itemScale.min +
    modelScale * (throwConfig.itemScale.max - throwConfig.itemScale.min);

  const eyeState = modelConfig.closeEyes ? 1 : modelConfig.openEyes ? 2 : 0;

  const randScale = (modelPosition.size + 100) / 200;
  const randH = (Math.random() * 100 - 50) * randScale;
  const randV = (Math.random() * 100 - 50) * randScale;
}

function createThrownImage(
  config: ImageConfig,
  image: HTMLImageElement,
  sizeScale: number,
  angle: number,
  spinSpeed: MinMax
) {
  const elm = document.createElement("img");
  elm.classList.add("animated");
  elm.style.width = `${image.width * config.scale * sizeScale}px`;
  elm.style.height = `${image.height * config.scale * sizeScale}px`;
  elm.style.imageRendering = config.pixel ? "pixelated" : "auto";

  // Animation speed is constant,
  if (spinSpeed.max - spinSpeed.min === 0) {
    return;
  }

  const clockwise = randomBool();

  elm.style.animationName = clockwise
    ? "spinClockwise"
    : "spinCounterClockwise";
}

/**
 * Loads the resources a throwable depends on (Image and optionally audio)
 *
 * @param imageConfig The image configuration
 * @param soundConfig The sound configuration
 * @returns
 */
async function loadThrowable(
  imageConfig: ImageConfig,
  soundConfig: SoundConfig
): Promise<{ image: HTMLImageElement | null; audio: HTMLAudioElement | null }> {
  let loadedImage: HTMLImageElement | null = null;
  let loadedAudio: HTMLAudioElement | null = null;

  // Load the image and audio if present
  await Promise.allSettled([
    // Load the image
    async () => {
      loadedImage = await loadImage(imageConfig.src);
    },

    // Load the sound
    async () => {
      if (!soundConfig) return;
      loadedAudio = await loadAudio(soundConfig.src);
    },
  ]);

  return { image: loadedImage, audio: loadedAudio };
}

async function loadImage(src: string): Promise<HTMLImageElement> {
  const image = new Image();
  image.src = src;

  return new Promise((resolve, reject) => {
    image.onload = () => resolve(image);
    image.onerror = () => reject();
  });
}

async function loadAudio(src: string): Promise<HTMLAudioElement> {
  const audio = new Audio(src);

  return new Promise((resolve, reject) => {
    audio.oncanplaythrough = () => resolve(audio);
    audio.onerror = () => reject();
  });
}
