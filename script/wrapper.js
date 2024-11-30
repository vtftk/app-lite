/**
 * This is a wrapper script injected into runnable scripts
 * to provide the primitives and helpers for scripting
 *
 * See api.d.ts for type definitions exposed to app
 */

const eventHandlers = {};

// API functions provided to the runtime
const api = {
  twitch: {
    sendChat: Deno.core.ops.op_twitch_send_chat,
  },
  http: {
    get: Deno.core.ops.op_http_get,
  },
};

function on(key, callback) {
  if (!eventHandlers[key]) {
    eventHandlers[key] = [];
  }

  eventHandlers[key].push(callback);
}

// Called by script runtime to invoke an event handler
function _triggerEvent({ type, data }) {
  if (eventHandlers[type]) {
    eventHandlers[type].forEach((callback) => callback(data));
  }
}

// Called by script runtime to determine which events are used
function _getEvents() {
  return Object.keys(eventHandlers);
}
