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
  return args.map((arg) => argToStr(arg)).join(" ");
}

function argToStr(value) {
  if (value === undefined) return "undefined";
  if (value === null) return "null";
  return JSON.stringify(value, Object.getOwnPropertyNames(value));
}

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
    return Promise.resolve(); // No handlers, resolve immediately
  }

  // Collect promises from all callbacks, handling both sync and async cases
  const promises = eventHandlers[type].map((callback) => {
    try {
      const result = callback(data);
      if (result instanceof Promise) {
        return result;
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

function createCommand(options) {
  on("chat", async (event) => {
    const fullMessage = event.message;
    const args = fullMessage.split(" ");

    if (options.command === undefined) {
      throw new Error("Command not specified");
    }

    const firstArg = args[0];
    const prefix = options.command;

    // Ignore non matching prefix
    if (firstArg.toLowerCase() !== prefix.toLowerCase()) {
      return;
    }

    const withoutPrefix = fullMessage.substring(prefix.length).trim();

    const user = {
      id: event.user_id,
      name: event.user_name,
      display_name: event.display_name,
    };

    // Check VIP access
    if (options.requireVip) {
      if (!(await api.twitch.isVip(user.id))) {
        return;
      }
    }

    // Check moderator access
    if (options.requireMod) {
      if (!(await api.twitch.isModerator(user.id))) {
        return;
      }
    }

    if (options.handle === undefined) {
      throw new Error("Handle is not defined for command");
    }

    const result = options.handle({
      fullMessage,
      message: withoutPrefix,
      user,
      args: args.slice(1),
    });

    const value = result instanceof Promise ? await result : result;

    if (typeof value === "string") {
      await api.twitch.sendChat(value);
    }
  });
}

// API functions provided to the runtime
const api = {
  twitch: {
    sendChat: (message) => Deno.core.ops.op_twitch_send_chat(message),
    isModerator: (userId) => Deno.core.ops.op_twitch_is_mod(userId),
    isVip: (userId) => Deno.core.ops.op_twitch_is_vip(userId),
  },
  kv: {
    get: (key) => Deno.core.ops.op_kv_get(key),
    remove: (key) => Deno.core.ops.op_kv_remove(key),
    set: (key, value) => Deno.core.ops.op_kv_set(key, value),
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

globalThis.api = api;
globalThis.on = on;
globalThis.createCommand = createCommand;
globalThis._getEvents = _getEvents;
globalThis._triggerEvent = _triggerEvent;

globalThis.console = {
  log: info,
  info,
  error,
  debug,
};
