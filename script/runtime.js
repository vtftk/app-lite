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

// API functions provided to the runtime
const api = {
  twitch: {
    sendChat: (message) => Deno.core.ops.op_twitch_send_chat(message),
    isModerator: (userId) => Deno.core.ops.op_twitch_is_mod(userId),
    isVip: (userId) => Deno.core.ops.op_twitch_is_vip(userId),
    getUsernameArg: (arg) => getUsernameArg(arg),
  },
  kv: {
    get: (key) => Deno.core.ops.op_kv_get(key),
    remove: (key) => Deno.core.ops.op_kv_remove(key),
    set: (key, value) => Deno.core.ops.op_kv_set(key, value),

    setObject: (key, value) =>
      Deno.core.ops.op_kv_set(key, JSON.stringify(value)),
    getObject: async (key, defaultValue) => {
      const value = await Deno.core.ops.op_kv_get(key);
      if (value === null) return defaultValue ?? null;
      return JSON.parse(value);
    },
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
