/**
 * This is the core runtime script this is stored as a JS snapshot
 * and all the globals created by this script are exposed when
 * running scripts at runtime
 *
 * This contains helpers, wrapper functions and glue for interacting
 * with the Rust side of the runtime
 */

import * as logging from "./logging";
import * as kv from "./kv";
import * as twitch from "./twitch";
import * as http from "./http";
import * as integrations from "./integrations";
import * as vtftk from "./vtftk";

// API functions provided to the runtime
Object.defineProperty(globalThis, "api", {
  value: {
    twitch,
    kv,
    http,
    logging,
    vtftk,
    integrations,
  },
  writable: false,
  configurable: false,
});

// Copy the logging functions to the commonly known console functions
Object.defineProperty(globalThis, "console", {
  value: {
    log: logging.info,
    info: logging.info,
    error: logging.error,
    debug: logging.debug,
  },
  writable: false,
  configurable: false,
});

Array.prototype.random = function () {
  return this[Math.floor(Math.random() * this.length)];
};
