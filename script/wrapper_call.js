(() => {
  const userFunction = async (event) => {
    // Run user code
    try {
      USER_CODE;
    } catch (err) {
      console.error("error running user event script code", err);
    }
  };

  return async (ctx, data) => {
    api.logging.runWithContext(ctx, async () => {
      await userFunction(data);
    });
  };
})();
