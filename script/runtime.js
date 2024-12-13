/**
 * This is a wrapper script injected into runnable scripts
 * to provide the primitives and helpers for scripting
 *
 * See api.d.ts for type definitions exposed to app
 */

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

function isValidUsernameStrict(username) {
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
}

function getUsernameArg(arg, validate = false) {
  // Arg not provided
  if (arg === undefined || arg === null || typeof arg !== "string") return null;

  // Trim whitespace
  arg = arg.trim();

  // Strip @ from mention
  if (arg.startsWith("@")) arg = arg.substring(1);

  // Empty
  if (arg.length < 1) return null;

  // Apply strict validation
  if (validate && !isValidUsernameStrict(arg)) return null;

  return arg;
}

function kvSetText(key, value) {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "string") throw new Error("value must be a string");

  return Deno.core.ops.op_kv_set("Text", key, value);
}

async function kvGetText(key, defaultValue) {
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return value;
}

function kvGetRaw(key) {
  return Deno.core.ops.op_kv_get(key);
}

function kvRemove(key) {
  return Deno.core.ops.op_kv_remove(key);
}

function kvSetNumber(key, value) {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "number") throw new Error("value must be a number");

  return Deno.core.ops.op_kv_set("Number", key, value);
}

async function kvGetNumber(key, defaultValue) {
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return Number(value);
}

function kvSetArray(key, value) {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (!Array.isArray(value)) throw new Error("value must be an array");

  return Deno.core.ops.op_kv_set("Array", key, value);
}

async function kvGetArray(key, defaultValue) {
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return JSON.parse(value);
}

function kvSetObject(key, value) {
  if (typeof key !== "string") throw new Error("key must be a string");
  if (typeof value !== "object") throw new Error("value must be a object");

  return Deno.core.ops.op_kv_set("Object", key, JSON.stringify(value));
}

async function kvGetObject(key, defaultValue) {
  const value = await Deno.core.ops.op_kv_get(key);
  if (value === null) return defaultValue ?? null;
  return JSON.parse(value);
}

function createCounter(key) {
  const update = async (action) => {
    const value = await kvGetNumber(key, 0);
    const updated = action(value);
    await kvSetNumber(key, updated);
    return updated;
  };

  return {
    get: () => kvGetNumber(key, 0),
    set: (value) => kvSetNumber(key, value),
    increase: (amount) => update((value) => value + (amount ?? 1)),
    decrease: (amount) => update((value) => value - (amount ?? 1)),
  };
}

function createScopedCounter(key) {
  const update = async (scope, action) => {
    const objectValue = await kvGetObject(key, {});
    const value = objectValue[scope] ?? 0;
    const updated = action(value);
    objectValue[scope] = updated;
    await kvSetObject(key, objectValue);
    return updated;
  };

  return {
    get: async (scope) => {
      if (typeof scope !== "string") throw new Error("scope must be a string");
      const objectValue = await kvGetObject(key, {});
      return objectValue[scope] ?? 0;
    },
    set: async (scope, value) => {
      if (typeof scope !== "string") throw new Error("scope must be a string");
      if (typeof value !== "number") throw new Error("value must be a number");
      const objectValue = await kvGetObject(key, {});
      objectValue[scope] = value;
      return kvSetObject(key, objectValue);
    },
    increase: (scope, amount) =>
      update(scope, (value) => value + (amount ?? 1)),
    decrease: (scope, amount) =>
      update(scope, (value) => value - (amount ?? 1)),
    all: async () => {
      const objectValue = await kvGetObject(key, {});
      return Object.entries(objectValue).map(([scope, amount]) => ({
        scope,
        amount,
      }));
    },
  };
}

// API functions provided to the runtime
const api = {
  twitch: {
    sendChat: (message) => Deno.core.ops.op_twitch_send_chat(message),
    sendChatAnnouncement: (message, color) =>
      Deno.core.ops.op_twitch_send_chat_announcement(
        message,
        color ?? "primary"
      ),
    getUserByUsername: (username) =>
      Deno.core.ops.op_twitch_get_user_by_username(username),
    sendShoutout: (targetUserId) =>
      Deno.core.ops.op_twitch_send_shoutout(targetUserId),
    isModerator: (userId) => Deno.core.ops.op_twitch_is_mod(userId),
    isVip: (userId) => Deno.core.ops.op_twitch_is_vip(userId),
    getUsernameArg: (arg) => getUsernameArg(arg),
  },
  kv: {
    createCounter,
    createScopedCounter,

    getRaw: kvGetRaw,
    remove: kvRemove,

    setText: kvSetText,
    getText: kvGetText,
    setNumber: kvSetNumber,
    getNumber: kvGetNumber,
    setArray: kvSetArray,
    getArray: kvGetArray,
    setObject: kvSetObject,
    getObject: kvGetObject,
  },
  http: {
    get: (url) => Deno.core.ops.op_http_get(url),
  },
  logging: {
    debug,
    info,
    warn,
    error,
  },
  vtftk: {
    ttsVoices: () => Deno.core.ops.op_vtftk_tts_get_voices(),
    ttsGenerate: (req) => Deno.core.ops.op_vtftk_tts_generate(req),
    ttsGenerateParsed: (req) => Deno.core.ops.op_vtftk_tts_generate_parsed(req),
    playSound: (src, volume = 1) =>
      Deno.core.ops.op_vtftk_play_sound(src, volume),
    playSoundSeq: (sounds) => Deno.core.ops.op_vtftk_play_sound_seq(sounds),
  },
};

globalThis.api = api;
globalThis.console = {
  log: info,
  info,
  error,
  debug,
};
