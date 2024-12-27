const userFunction = async (event) => {
  // Run user code
  try {
    USER_CODE;
  } catch (err) {
    console.error("error running user event script code", err);
  }
};

async (ctx, data) => {
  api.logging.runWithContext(ctx, async () => {
    await userFunction(data);
  });
};
