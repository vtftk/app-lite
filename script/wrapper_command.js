async (ctx, cmd_ctx) => {
  api.logging.runWithContext(ctx, async () => {
    const userFunction = async (ctx) => {
      try {
        USER_CODE;
      } catch (err) {
        console.error("error running user command code", err);
      }
    };

    const commandCtx = {
      ...cmd_ctx,

      // Inject getters for helping with getting the target user
      get targetUser() {
        return api.twitch.getUsernameArg(cmd_ctx.args[0], false);
      },

      get targetUserValid() {
        return api.twitch.getUsernameArg(cmd_ctx.args[0], true);
      },
    };

    const value = await userFunction(commandCtx);
    if (typeof value === "string") {
      await api.twitch.sendChat(value);
    }
  });
};
