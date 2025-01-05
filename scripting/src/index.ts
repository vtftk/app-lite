/// <reference no-default-lib="true" />

/**
 * This is the core runtime script this is stored as a JS snapshot
 * and all the globals created by this script are exposed when
 * running scripts at runtime
 *
 * This contains helpers, wrapper functions and glue for interacting
 * with the Rust side of the runtime
 */

import * as dateFns from "date-fns";

import * as kv from "./kv";
import * as http from "./http";
import * as vtftk from "./vtftk";
import * as twitch from "./twitch";
import * as random from "./random";
import * as logging from "./logging";
import * as internal from "./internal";
import "./eventContext";
import "./commandContext";
import "./arrayExt";
import * as integrations from "./integrations";

/**
 * API provided for scripting functionality
 *
 * @interface API
 * @member twitch Twitch related APIs
 * @member kv Key value store APIs
 * @member http HTTP request APIs
 * @member logging Logging helpers
 * @member vtftk VTFTK specific helpers (Sounds, Throwables, VTube Studio Hotkeys)
 * @member integrations Integrations with external services (TTS Monster)
 * @member random Random generators (Numbers, Booleans, Array Items)
 * @member internal Internal VTFTK logic (Not intended for scripting use, used internally by the runtime)
 */
interface API {
  twitch: typeof twitch;
  kv: typeof kv;
  http: typeof http;
  logging: typeof logging;
  vtftk: typeof vtftk;
  integrations: typeof integrations;
  random: typeof random;
  internal: typeof internal;
}

/**
 * External modules embedded within the runtime for
 * eas of use
 *
 * @interface Modules
 * @member date-fns Date library for working with dates (https://date-fns.org/)
 */
interface Modules {
  "date-fns": typeof dateFns;
}

const _api: Readonly<API> = Object.freeze({
  twitch,
  kv,
  http,
  logging,
  vtftk,
  integrations,
  random,
  internal,
});

const _modules: Readonly<Modules> = Object.freeze({
  "date-fns": dateFns,
});

/**
 * Get a builtin module
 *
 * @param key The module key
 * @returns The module itself
 */
function _module<K extends keyof Modules>(key: K): Modules[K] {
  return _modules[key];
}

const _console = Object.freeze({
  log: logging.info,
  info: logging.info,
  error: logging.error,
  debug: logging.debug,
});

// Define global properties
Object.defineProperties(globalThis, {
  // Global module helper
  module: {
    value: _module,
    writable: false,
    configurable: false,
  },

  // Define API globals
  api: {
    value: _api,
    writable: false,
    configurable: false,
  },
  // Define console globals
  console: {
    value: _console,
    writable: false,
    configurable: false,
  },
});

declare global {
  export const api: API;
  export const console: typeof _console;
  export const module: typeof _module;
}
