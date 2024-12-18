async (ctx, cmd_ctx) => {
  api.logging.runWithContext(ctx, async () => {
    const userFunction = async (ctx) => {
      try {
        USER_CODE;
      } catch (err) {
        console.error("error running user command code", err);
      }
    };

    const value = await userFunction(cmd_ctx);
    if (typeof value === "string") {
      await api.twitch.sendChat(value);
    }
  });
};
