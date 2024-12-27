/**
 * This is the core runtime script this is stored as a JS snapshot
 * and all the globals created by this script are exposed when
 * running scripts at runtime
 *
 * This contains helpers, wrapper functions and glue for interacting
 * with the Rust side of the runtime
 */

const { AsyncVariable, setAsyncContext } = Deno.core;

// Async variable for storing logging context
const loggingContextVariable = new AsyncVariable();

/**
 * Converts any number of arguments into strings
 *
 * @param  {...any} args Arguments to stringify
 * @returns
 */
function stringifyValues(...args) {
  return args.map((arg) => stringifyValue(arg)).join(" ");
}

/**
 * Converts the provided value into a string
 *
 * @param {*} data Value to print
 * @returns The string version of the value
 */
function stringifyValue(data) {
  // Handle special cases
  if (data === undefined) return "undefined";
  if (data === null) return "null";
  if (typeof data === "string") return data;

  const seen = [];
  const keys = [];

  function stringify(key, value) {
    if (typeof value === "object" && value) {
      let index = seen.indexOf(value);

      if (index !== -1) {
        let topkey = keys[index];
        const path = [topkey];

        for (index--; index > 0; index--) {
          if (seen[index][topkey] === value) {
            value = seen[index];
            path.unshift((topkey = keys[index]));
          }
        }

        return "<ref:" + path.join(".") + ">";
      }

      seen.push(value);
      keys.push(key);
    }

    return value;
  }

  return JSON.stringify(data, stringify, 2);
}

const logging = {
  // Helper to run a callback within a specific async context
  runWithContext: (ctx, callback, ...args) => {
    const previous = loggingContextVariable.enter(ctx);
    try {
      return Reflect.apply(callback, null, args);
    } finally {
      setAsyncContext(previous);
    }
  },

  getContext: () => {
    return loggingContextVariable.get();
  },

  info: (...args) => {
    const ctx = logging.getContext();
    Deno.core.ops.op_log_info(ctx, stringifyValues(...args));
  },
  error: (...args) => {
    const ctx = logging.getContext();
    Deno.core.ops.op_log_error(ctx, stringifyValues(...args));
  },
  warn: (...args) => {
    const ctx = logging.getContext();
    Deno.core.ops.op_log_warn(ctx, stringifyValues(...args));
  },
  debug: (...args) => {
    const ctx = logging.getContext();
    Deno.core.ops.op_log_debug(ctx, stringifyValues(...args));
  },
};

const kv = {
  setText: (key, value) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    if (typeof value !== "string") throw new Error("value must be a string");

    return Deno.core.ops.op_kv_set("Text", key, value);
  },

  getText: async (key, defaultValue) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null) return defaultValue ?? null;
    return value;
  },

  getRaw: (key) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    return Deno.core.ops.op_kv_get(key);
  },

  remove: (key) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    return Deno.core.ops.op_kv_remove(key);
  },

  setNumber: (key, value) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    if (typeof value !== "number") throw new Error("value must be a number");

    return Deno.core.ops.op_kv_set("Number", key, value);
  },

  getNumber: async (key, defaultValue) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null) return defaultValue ?? null;
    return Number(value);
  },

  setArray: (key, value) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    if (!Array.isArray(value)) throw new Error("value must be an array");

    return Deno.core.ops.op_kv_set("Array", key, value);
  },

  getArray: async (key, defaultValue) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null) return defaultValue ?? null;
    return JSON.parse(value);
  },

  setObject: (key, value) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    if (typeof value !== "object") throw new Error("value must be a object");

    return Deno.core.ops.op_kv_set("Object", key, JSON.stringify(value));
  },

  getObject: async (key, defaultValue) => {
    if (typeof key !== "string") throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null) return defaultValue ?? null;
    return JSON.parse(value);
  },

  createCounter: (key) => {
    if (typeof key !== "string") throw new Error("key must be a string");

    const update = async (action) => {
      const value = await kv.getNumber(key, 0);
      const updated = action(value);
      await kv.setNumber(key, updated);
      return updated;
    };

    return {
      get: () => kv.getNumber(key, 0),
      set: (value) => kv.setNumber(key, value),
      increase: (amount) => update((value) => value + (amount ?? 1)),
      decrease: (amount) => update((value) => value - (amount ?? 1)),
    };
  },

  createScopedCounter: (key) => {
    if (typeof key !== "string") throw new Error("key must be a string");

    const update = async (scope, action) => {
      const objectValue = await kv.getObject(key, {});
      const value = objectValue[scope] ?? 0;
      const updated = action(value);
      objectValue[scope] = updated;
      await kv.setObject(key, objectValue);
      return updated;
    };

    return {
      get: async (scope) => {
        if (typeof scope !== "string")
          throw new Error("scope must be a string");
        const objectValue = await kv.getObject(key, {});
        return objectValue[scope] ?? 0;
      },
      set: async (scope, value) => {
        if (typeof scope !== "string")
          throw new Error("scope must be a string");
        if (typeof value !== "number")
          throw new Error("value must be a number");
        const objectValue = await kv.getObject(key, {});
        objectValue[scope] = value;
        return kv.setObject(key, objectValue);
      },
      increase: (scope, amount) =>
        update(scope, (value) => value + (amount ?? 1)),
      decrease: (scope, amount) =>
        update(scope, (value) => value - (amount ?? 1)),
      all: async () => {
        const objectValue = await kv.getObject(key, {});
        return Object.entries(objectValue).map(([scope, amount]) => ({
          scope,
          amount,
        }));
      },
    };
  },
};

const twitch = {
  sendChat: (message) => Deno.core.ops.op_twitch_send_chat(message),
  sendChatAnnouncement: (message, color) =>
    Deno.core.ops.op_twitch_send_chat_announcement(message, color ?? "primary"),
  getUserByUsername: (username) =>
    Deno.core.ops.op_twitch_get_user_by_username(username),
  sendShoutout: (targetUserId) =>
    Deno.core.ops.op_twitch_send_shoutout(targetUserId),
  isModerator: (userId) => Deno.core.ops.op_twitch_is_mod(userId),
  isVip: (userId) => Deno.core.ops.op_twitch_is_vip(userId),
  getUsernameArg: (arg, validate = false) => {
    // Arg not provided
    if (arg === undefined || arg === null || typeof arg !== "string")
      return null;

    // Trim whitespace
    arg = arg.trim();

    // Strip @ from mention
    if (arg.startsWith("@")) arg = arg.substring(1);

    // Empty
    if (arg.length < 1) return null;

    // Apply strict validation
    if (validate && !twitch.isValidUsernameStrict(arg)) return null;

    return arg;
  },

  isValidUsernameStrict: (username) => {
    if (!username) return false;

    const length = username.length;

    // Check length
    if (length < 4 || length > 25) return false;

    // Check for leading or trailing underscores
    if (username[0] === "_" || username[length - 1] === "_") return false;

    // Iterate through characters to validate
    for (let i = 0; i < length; i++) {
      const char = username[i];

      // Check if character is valid (alphanumeric or underscore)
      const isAlphaNumeric =
        (char >= "a" && char <= "z") ||
        (char >= "A" && char <= "Z") ||
        (char >= "0" && char <= "9") ||
        char === "_";

      if (!isAlphaNumeric) return false;
    }

    return true;
  },
};

async function createHttpRequest(options) {
  // URL must be defined and a string
  if (options.url === undefined || typeof options.url !== "string") {
    throw new Error("url must be a present and a string");
  }

  let requestBody = undefined;
  const body = options.body;
  if (typeof body === "string") {
    requestBody = { type: "text", value: body };
  } else if (typeof body === "object") {
    requestBody = { type: "json", value: body };
  }

  let responseFormat = (options.responseFormat ?? "text").toLowerCase();

  const response = await Deno.core.ops.op_http_request({
    url: options.url,
    method: options.method,
    body: requestBody,
    headers: options.headers,
    timeout: options.timeout,
    response_format: responseFormat,
  });

  return {
    ...response,

    get ok() {
      return Math.floor(response.status / 100) == 2;
    },
  };
}

const http = {
  request: (options) => createHttpRequest(options),
  get: (url, options) => createHttpRequest({ ...options, url, method: "GET" }),
  post: (url, body, options) =>
    createHttpRequest({ ...options, url, method: "POST", body }),
  put: (url, body, options) =>
    createHttpRequest({ ...options, url, method: "PUT", body }),
  patch: (url, body, options) =>
    createHttpRequest({ ...options, url, method: "PATCH", body }),
  delete: (url, body, options) =>
    createHttpRequest({ ...options, url, method: "DELETE", body }),
};

const vtftk = {
  ttsVoices: () => Deno.core.ops.op_vtftk_tts_get_voices(),
  ttsGenerate: (req) => Deno.core.ops.op_vtftk_tts_generate(req),
  ttsGenerateParsed: (voice_id, message) =>
    Deno.core.ops.op_vtftk_tts_generate_parsed(voice_id, message),
  playSound: (src, volume = 1) =>
    Deno.core.ops.op_vtftk_play_sound(src, volume),
  playSoundSeq: (sounds) => Deno.core.ops.op_vtftk_play_sound_seq(sounds),
};

// API functions provided to the runtime
globalThis.api = {
  twitch,
  kv,
  http,
  logging,
  vtftk,
};

// Copy the logging functions to the commonly known console functions
globalThis.console = {
  log: logging.info,
  info: logging.info,
  error: logging.error,
  debug: logging.debug,
};
