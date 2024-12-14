(() => {
  const userFunction = async (ctx) => {
    try {
      USER_CODE;
    } catch (err) {
      console.error("error running user command code", err);
    }
  };

  return async (ctx, cmd_ctx) => {
    _asyncLocalStorage.run(ctx, async () => {
      const value = await userFunction(cmd_ctx);
      if (typeof value === "string") {
        await api.twitch.sendChat(value);
      }
    });
  };
})();
