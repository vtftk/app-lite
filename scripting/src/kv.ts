/**
 * Store a string value within the KV store
 *
 * @param key The key to store the value under
 * @param value The string value to store
 * @returns Promise resolved when the value is stored
 */
export function setText(key: string, value: string): Promise<void> {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "string") throw new Error("value must be a string");

  return Deno.core.ops.op_kv_set("Text", key, value);
}

/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
export async function getText(
  key: string,
  defaultValue: string
): Promise<string>;

/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
export async function getText(key: string): Promise<string | null>;

/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the text value, null if there is no value and no default is specified
 */
export async function getText(
  key: string,
  defaultValue?: string
): Promise<string | null> {
  if (typeof key !== "string") throw new Error("key must be a string");
  const value: string | null = await Deno.core.ops.op_kv_get(key);
  if (value === null && defaultValue !== undefined) return defaultValue;
  return value;
}

/**
 * Remove a key value pair from the KV store
 *
 * @param key The key to remove
 * @returns Promise resolved when the value is removed
 */
export function remove(key: string): Promise<void> {
  if (typeof key !== "string") throw new Error("key must be a string");
  return Deno.core.ops.op_kv_remove(key);
}

/**
 * Store a number value within the KV store
 *
 * @param key The key to store the value under
 * @param value The number value to store
 * @returns Promise resolved when the value is stored
 */
export function setNumber(key: string, value: number): Promise<void> {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "number") throw new Error("value must be a number");

  return Deno.core.ops.op_kv_set("Number", key, value);
}

/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
export async function getNumber(
  key: string,
  defaultValue: number
): Promise<number>;

/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
export async function getNumber(key: string): Promise<number | null>;

/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the number value, null if there is no value and no default is specified
 */
export async function getNumber(
  key: string,
  defaultValue?: number
): Promise<number | null> {
  if (typeof key !== "string") throw new Error("key must be a string");
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return Number(value);
}

/**
 * Store an array value within the KV store
 *
 * @param key The key to store the value under
 * @param value The array value to store
 * @returns Promise resolved when the value is stored
 */
export function setArray<T>(key: string, value: T[]): Promise<void> {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (!Array.isArray(value)) throw new Error("value must be an array");

  return Deno.core.ops.op_kv_set("Array", key, value);
}

/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
export async function getArray<T>(key: string, defaultValue: T[]): Promise<T[]>;

/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
export async function getArray<T>(key: string): Promise<T[] | null>;

/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the array value, null if there is no value and no default is specified
 */
export async function getArray<T>(
  key: string,
  defaultValue?: T[]
): Promise<T[] | null> {
  if (typeof key !== "string") throw new Error("key must be a string");
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return JSON.parse(value);
}

/**
 * Store an object value within the KV store
 *
 * @param key The key to store the value under
 * @param value The object value to store
 * @returns Promise resolved when the value is stored
 */
export function setObject<T>(key: string, value: T): Promise<void> {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "object") throw new Error("value must be a object");

  return Deno.core.ops.op_kv_set("Object", key, JSON.stringify(value));
}

/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
export async function getObject<T>(key: string, defaultValue: T): Promise<T>;

/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
export async function getObject<T>(key: string): Promise<T | null>;

/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the object value, null if there is no value and no default is specified
 */
export async function getObject<T>(
  key: string,
  defaultValue?: T
): Promise<T | null> {
  if (typeof key !== "string") throw new Error("key must be a string");
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return JSON.parse(value);
}

type Transform<T> = (value: T) => T;

export interface Counter {
  /**
   * Get the current counter value
   *
   * @returns Promise resolved to the current counter value
   */
  get: () => Promise<number>;

  /**
   * Set the counter to a specific value
   *
   * @param value The value to set the counter to
   * @returns Promise resolved when the counter is updated
   */
  set: (value: number) => Promise<void>;

  /**
   * Increase the counter by the provided amount, defaults to 1
   *
   * @param amount Amount to increase by (Defaults to 1)
   * @returns Promise resolved to the new counter value
   */
  increase: (amount?: number) => Promise<number>;

  /**
   * Decrease the counter by the provided amount, defaults to 1
   *
   * @param amount Amount to increase by (Defaults to 1)
   * @returns Promise resolved to the new counter value
   */
  decrease: (amount?: number) => Promise<number>;
}

/**
 * Create a new counter using the provided key
 *
 * @param key The key to store the counter value within
 * @returns The created counter
 */
export function createCounter(key: string): Counter {
  if (typeof key !== "string") throw new Error("key must be a string");

  const update = async (action: Transform<number>) => {
    const value = await getNumber(key, 0);
    const updated = action(value);
    await setNumber(key, updated);
    return updated;
  };

  return {
    get: () => getNumber(key, 0),
    set: (value: number) => setNumber(key, value),
    increase: (amount?: number) => update((value) => value + (amount ?? 1)),
    decrease: (amount?: number) => update((value) => value - (amount ?? 1)),
  };
}

// Internal structure used to store scoped counters (Within KV)
export type ScopedCounterObject = Partial<Record<string, number>>;

export interface ScopedCounterEntry {
  // The scope (i.e the user name)
  scope: string;
  // Current amount for the scope
  amount: number;
}

export interface ScopedCounter {
  /**
   * Get the counter value for the provided scope
   *
   * @param scope The scope to get the counter for (i.e the user name)
   * @returns Promise resolved to the scope value
   */
  get(scope: string): Promise<number>;

  /**
   * Set the counter value for a specific scope
   *
   * @param scope The scope to get the counter for (i.e the user name)
   * @param value The value to set for the scope
   * @returns Promise resolved when the value is updated
   */
  set(scope: string, value: number): Promise<void>;

  /**
   * Increase the counter value for a specific scope
   *
   * @param scope The scope to get the counter for (i.e the user name)
   * @param amount Amount to increase the counter by (Default: 1)
   * @returns Promise resolved when the value is updated returns the new value after the update
   */
  increase(scope: string, amount?: number): Promise<number>;

  /**
   * Decrease the counter value for a specific scope
   *
   * @param scope The scope to get the counter for (i.e the user name)
   * @param amount Amount to decrease the counter by (Default: 1)
   * @returns Promise resolved when the value is updated returns the new value after the update
   */
  decrease(scope: string, amount?: number): Promise<number>;

  /**
   * Gets all entries within the scoped counter
   *
   * @returns Promise resolved to the list of entries
   */
  all(): Promise<ScopedCounterEntry[]>;
}

/**
 * Create a new scoped counter using the provided key
 *
 * Scoped counters provide a way to track a counter for a specific "scope"
 * this can be used to create per-user counters or per-game counters
 *
 * @param key The key to store the counter value within
 * @returns The created scoped counter
 */
export function createScopedCounter(key: string): ScopedCounter {
  if (typeof key !== "string") throw new Error("key must be a string");

  /**
   * Updates the value at the provided scope returning
   * the new value
   *
   * @param scope The scope to update
   * @param action The action to transform the value
   * @returns Promise resolved to the new value
   */
  const update = async (scope: string, action: Transform<number>) => {
    const objectValue = await getObject<any>(key, {});
    const value = objectValue[scope] ?? 0;
    const updated = action(value);
    objectValue[scope] = updated;
    await setObject(key, objectValue);
    return updated;
  };

  return {
    get: async (scope: string) => {
      if (typeof scope !== "string") throw new Error("scope must be a string");
      const objectValue = await getObject<ScopedCounterObject>(key, {});
      return objectValue[scope] ?? 0;
    },
    set: async (scope: string, value: number) => {
      if (typeof scope !== "string") throw new Error("scope must be a string");
      if (typeof value !== "number") throw new Error("value must be a number");
      const objectValue = await getObject<ScopedCounterObject>(key, {});
      objectValue[scope] = value;
      return setObject(key, objectValue);
    },
    increase: (scope: string, amount?: number) =>
      update(scope, (value) => value + (amount ?? 1)),
    decrease: (scope: string, amount?: number) =>
      update(scope, (value) => value - (amount ?? 1)),
    all: async () => {
      const objectValue = await getObject<ScopedCounterObject>(key, {});
      return Object.entries(objectValue).map(([scope, amount]) => ({
        scope,
        amount: amount!,
      }));
    },
  };
}
