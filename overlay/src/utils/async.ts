export async function sleep(duration: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, duration));
}

/**
 * Runs a function on an interval
 *
 * @param action The action to run
 * @param interval The interval between each run
 * @param maxIterations The maximum number of times to run
 * @returns Promise for when its complete
 */
export async function executeInterval(
  action: VoidFunction,
  interval: number,
  maxIterations: number
): Promise<void> {
  return new Promise((resolve) => {
    let intervalHandle: number;
    let iteration: number = 0;

    intervalHandle = setInterval(() => {
      action();
      iteration += 1;

      if (iteration === maxIterations) {
        clearInterval(intervalHandle);
        resolve();
      }
    }, interval);
  });
}

export async function loadImage(src: string): Promise<HTMLImageElement> {
  const image = new Image();
  image.src = src;

  return new Promise((resolve, reject) => {
    image.onload = () => resolve(image);
    image.onerror = (err) => reject(err);
  });
}

export async function loadAudio(src: string): Promise<HTMLAudioElement> {
  const audio = new Audio(src);

  return new Promise((resolve, reject) => {
    audio.oncanplaythrough = () => resolve(audio);
    audio.onerror = () => reject();
  });
}
