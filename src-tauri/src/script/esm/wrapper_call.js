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
    api.internal.runWithContext(ctx, async () => {
      await userFunction(data);
    });
  };
})();
