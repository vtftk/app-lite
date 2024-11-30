/**
 * This is a wrapper script injected into runnable scripts
 * to provide the primitives and helpers for scripting
 *
 * See api.d.ts for type definitions exposed to app
 */

const eventHandlers = {};

function info(...args) {
  Deno.core.ops.op_log_info(`${argsToMessage(...args)}\n`);
}

function error(...args) {
  Deno.core.ops.op_log_error(`${argsToMessage(...args)}\n`);
}

function warn(...args) {
  Deno.core.ops.op_log_warn(`${argsToMessage(...args)}\n`);
}

function debug(...args) {
  Deno.core.ops.op_log_debug(`${argsToMessage(...args)}\n`);
}

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

// API functions provided to the runtime
const api = {
  twitch: {
    sendChat: Deno.core.ops.op_twitch_send_chat,
  },
  http: {
    get: (url) => {
      const promise = Deno.core.ops.op_http_get(url);

      return promise.then((result) => {
        console.info("Promise http resolved", result);
        return result;
      });
    },
  },
  logging: {
    debug,
    info,
    warn,
    error,
  },
};

globalThis.console = {
  log: info,
  info,
  error,
  debug,
};

function on(key, callback) {
  if (!eventHandlers[key]) {
    eventHandlers[key] = [];
  }

  api.logging.info("subscribed to " + key);
  eventHandlers[key].push(callback);
}

// Called by script runtime to invoke an event handler
async function _triggerEvent({ type, data }) {
  if (eventHandlers[type] === undefined) {
    api.logging.info("no event handlers to run", type, eventHandlers);
    return Promise.resolve(); // No handlers, resolve immediately
  }

  api.logging.info("running event handlers");

  // Collect promises from all callbacks, handling both sync and async cases
  const promises = eventHandlers[type].map((callback) => {
    try {
      api.logging.info("running event handler");
      const result = callback(data);
      api.logging.info("running event handler", result);
      if (result instanceof Promise) {
        return result.then((resolved) => {
          api.logging.info("promise resolved");
          return resolved;
        });
      } else {
        return Promise.resolve(result);
      }
    } catch (error) {
      console.error(`Error in callback for event "${type}":`, error);
      return Promise.resolve(); // Return a resolved promise on error
    }
  });

  try {
    // Wait for all promises to resolve
    await Promise.all(promises);
  } catch (error) {
    console.error(`Error in callback for event "${type}":`, error);
    return Promise.resolve(); // Return a resolved promise on error
  }
}

// Called by script runtime to determine which events are used
function _getEvents() {
  return Object.keys(eventHandlers);
}
