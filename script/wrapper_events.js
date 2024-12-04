(() => {
  const eventNames = new Set();

  function on(key, _callback) {
    eventNames.add(key);
  }

  globalThis.on = on;

  try {
    USER_CODE;
  } catch (err) {
    console.error("error running user script code", err);
  }

  delete globalThis.on;

  return Array.from(eventNames);
})()
