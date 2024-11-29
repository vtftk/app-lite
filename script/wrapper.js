const eventHandlers = {};

function on(key, callback) {
  if (!eventHandlers[key]) {
    eventHandlers[key] = [];
  }
  eventHandlers[key].push(callback);
}

function _triggerEvent(key, event) {
  if (eventHandlers[key]) {
    eventHandlers[key].forEach((callback) => callback(event));
  }
}
