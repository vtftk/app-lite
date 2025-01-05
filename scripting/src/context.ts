const { AsyncVariable, setAsyncContext } = Deno.core;

// Async variable for storing context
const contextVariable = new AsyncVariable();

/**
 * Runs the provided function within the specific context
 *
 * @internal
 *
 * @param ctx The logging context
 * @param callback The function to run
 * @param args Arguments for the function
 */
export function runWithContext<C, A extends unknown[], R>(
  ctx: C,
  callback: (...args: A) => R,
  ...args: A
): R {
  const previous = contextVariable.enter(ctx);
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
  return contextVariable.get();
}
