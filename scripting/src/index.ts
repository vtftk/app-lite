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
import * as core from "./core";
import * as vtftk from "./vtftk";
import * as twitch from "./twitch";
import * as random from "./random";
import * as logging from "./logging";
import * as internal from "./internal";
import "./eventContext";
import "./commandContext";
import "./arrayExt";
import * as integrations from "./integrations";

type TwitchAPI = typeof twitch;
type KeyValueAPI = typeof kv;
type HttpAPI = typeof http;
type LoggingAPI = typeof logging;
type VTFTKAPI = typeof vtftk;
type IntegrationsAPI = typeof integrations;
type RandomAPI = typeof random;
type InternalAPI = typeof internal;
type CoreAPI = typeof core;

interface API {
  /**
   * Twitch - Work with twitch related functionality. Access chat, get user details
   */
  twitch: TwitchAPI;
  /**
   * Key Value Store - Store and access persisted data like counters.
   */
  kv: KeyValueAPI;
  /**
   * HTTP - Work with HTTP requests, send HTTP requests
   */
  http: HttpAPI;
  /**
   * Logging - Log information to the persisted logging data
   */
  logging: LoggingAPI;
  /**
   * VTFTK - Access VTFTK functionality like throwing items, and playing sounds
   */
  vtftk: VTFTKAPI;
  /**
   * Integrations - Access external integrations like TTS Monster
   */
  integrations: IntegrationsAPI;
  /**
   * Random - Helpers for working with randomness such as getting random numbers or random array items
   */
  random: RandomAPI;
  /**
   * Internal - Internals used by VTFTK, not intended for use
   * @internal
   */
  internal: InternalAPI;
  /**
   * Core - Provides some useful core features used internally like UUIDs
   */
  core: CoreAPI;
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
  core,
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
  /**
   * API provided for scripting functionality
   */
  export const api: API;
  export const console: typeof _console;
  export const module: typeof _module;
}
