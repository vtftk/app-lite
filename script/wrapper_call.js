(() => {
  const __eventHandlers = {};

  (() => {
    function on(key, callback) {
      if (!__eventHandlers[key]) {
        __eventHandlers[key] = [];
      }

      __eventHandlers[key].push(callback);
    }

    try {
      USER_CODE;
    } catch (err) {
      console.error("error running user script code", err);
    }
  })();

  return async (ctx, type, data) => {
    _asyncLocalStorage.run(ctx, async () => {
      const handler = __eventHandlers[type];

      if (handler === undefined) {
        return;
      }

      // Wait for all promises to resolve
      const results = await Promise.allSettled(
        handler.map((callback) => callback(data))
      );

      // Log out all failures
      for (const result of results) {
        if (result.status === "rejected") {
          const reason = result.reason;
          console.error(`Error in callback for event "${type}":`, reason);
        }
      }
    });
  };
})();
