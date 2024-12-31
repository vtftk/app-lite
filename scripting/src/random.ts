/**
 * Returns a random number between the provided minimum
 * and maximum values
 *
 * Number may include a decimal value use randomInteger to
 * only get whole numbers
 *
 *
 * @param min The minimum value
 * @param max The maximum value
 * @returns The random number
 */
export function randomNumber(min: number, max: number): number {
  return min + Math.random() * (max - min);
}

/**
 * Returns a random number between the provided minimum
 * and maximum values
 *
 * @param min The minimum value
 * @param max The maximum value
 * @returns The random number
 */
export function randomInteger(min: number, max: number): number {
  return Math.floor(min + Math.random() * (max - min));
}

/**
 * Returns a random boolean value
 *
 * @returns The random boolean value
 */
export function randomBoolean(): boolean {
  return Math.random() < 0.5;
}

/**
 * Picks a random item from the provided array
 *
 * If the array is empty the returned value will be
 * undefined
 *
 * @param array The array to pick from
 * @returns The random item
 */
export function randomArrayItem<T>(array: T[]): T {
  return array[Math.floor(Math.random() * array.length)];
}
