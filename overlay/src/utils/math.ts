export function randomRange(min: number, max: number) {
  return Math.random() * (max - min) + min;
}

export function percentRange(value: number, min: number, max: number): number {
  return value * (max - min) + min;
}

export function arrayRandom<T>(array: T[]): T {
  if (array.length < 1)
    throw new Error("cannot choose random item from empty array");
  return array[Math.floor(Math.random() * array.length)];
}

export function randomBool(): boolean {
  return Math.random() < 0.5;
}
