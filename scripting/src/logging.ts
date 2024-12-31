/* eslint-disable @typescript-eslint/no-explicit-any */
const { AsyncVariable, setAsyncContext } = Deno.core;

// Async variable for storing logging context
const loggingContextVariable = new AsyncVariable();

/**
 * Runs the provided function within the specific logging context
 *
 * @internal
 *
 * @param ctx The logging context
 * @param callback The function to run
 * @param args Arguments for the function
 */
export function runWithContext<C, A extends any[], R>(
  ctx: C,
  callback: (...args: A) => R,
  ...args: A
): R {
  const previous = loggingContextVariable.enter(ctx);
  try {
    return Reflect.apply(callback, null, args);
  } finally {
    setAsyncContext(previous);
  }
}

/**
 * Get the current logging context set by {@see runWithContext}
 *
 * @returns The current context or undefined if not within a context
 */
export function getContext<T>(): T {
  return loggingContextVariable.get();
}

/**
 * Log the provided arguments at the "INFO" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
export function info(...args: unknown[]) {
  _log("Info", ...args);
}

/**
 * Log the provided arguments at the "ERROR" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
export function error(...args: unknown[]) {
  _log("Error", ...args);
}

/**
 * Log the provided arguments at the "WARN" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
export function warn(...args: unknown[]) {
  _log("Warn", ...args);
}

/**
 * Log the provided arguments at the "DEBUG" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
export function debug(...args: unknown[]) {
  _log("Debug", ...args);
}

/**
 * Internal logging function calls the Deno op to trigger
 * logging on the Rust end
 *
 * @param level The log level
 * @param args Arguments to log
 */
function _log(
  level: "Info" | "Error" | "Warn" | "Debug",
  ...args: unknown[]
): void {
  const ctx = getContext();
  Deno.core.ops.op_log(ctx, level, stringifyArgs(...args));
}

/**
 * Stringify a collection of arguments for logging
 *
 * @param args Arguments to stringify
 * @returns The stringified arguments
 */
function stringifyArgs(...args: unknown[]): string {
  return args.map((arg) => stringify(arg)).join(" ");
}

/**
 * Deeply convert a value to a string, handles self referencing
 * objects by replacing them with <ref:{path}>
 *
 * @param data Value to print
 * @returns The string version of the value
 */
function stringify(data: unknown): string {
  // Handle special cases
  if (data === undefined) return "undefined";
  if (data === null) return "null";
  if (typeof data === "string") return data;
  if (data instanceof Error) {
    return JSON.stringify(data, Object.getOwnPropertyNames(data));
  }

  const seen: any[] = [];
  const keys: string[] = [];

  function stringify(key: string, value: any): any {
    // Skip non or null/undefined objects
    if (typeof value !== "object" || !value) return value;

    let index = seen.indexOf(value);

    // Have not seen the value yet
    if (index === -1) {
      seen.push(value);
      keys.push(key);
      return value;
    }

    // Build the reference path for previously seen objects
    let topKey = keys[index];
    const path = [topKey];

    // Trace back to find the full path of the circular reference
    for (index--; index > 0; index--) {
      if (seen[index][topKey] === value) {
        value = seen[index];
        topKey = keys[index];
        path.unshift(topKey);
      }
    }

    return "<ref:" + path.join(".") + ">";
  }

  return JSON.stringify(data, stringify, 2);
}
