(() => {
  const _eventHandlers = {};

  function on(key, callback) {
    if (!_eventHandlers[key]) {
      _eventHandlers[key] = [];
    }

    _eventHandlers[key].push(callback);
  }

  globalThis.on = on;

  try {
    USER_CODE;
  } catch (err) {
    console.error("error running user script code", err);
  }

  delete globalThis.on;

  return Object.keys(_eventHandlers);
})()
