(() => {
  const userFunction = async (ctx) => {
    try {
      USER_CODE;
    } catch (err) {
      console.error("error running user command code", err);
    }
  };

  return async (ctx, cmd_ctx) => {
    const { runWithContext, extendCommandContext } = api.internal;

    runWithContext(ctx, async () => {
      const commandCtx = extendCommandContext(cmd_ctx);
      const value = await userFunction(commandCtx);

      if (typeof value === "string") {
        await api.twitch.sendChat(value);
      }
    })
  };
})();
