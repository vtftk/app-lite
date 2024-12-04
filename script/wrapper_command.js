(() => {
  const userFunction = async (ctx) => {
    try {
      USER_CODE;
    } catch (err) {
      console.error("error running user command code", err);
    }
  };

  return async (ctx) => {
    const value = await userFunction(ctx);
    if (typeof value === "string") {
      await api.twitch.sendChat(value);
    }
  };
})()
