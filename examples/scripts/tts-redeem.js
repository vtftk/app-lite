on("redeem", async (event) => {
  if (event.reward_name !== "TTS") {
    return;
  }

  const message = event.user_input;

  // Acknowledge the request
  api.twitch.sendChat("Generating TTS message.....");

  // Generate the TTS message
  const urls = await api.vtftk.ttsGenerateParsed(message);

  // Play the TTS message sounds
  await api.vtftk.playSoundSeq(
    urls.map((url) => ({
      src: url,
      volume: 1,
    }))
  );

  await api.twitch.sendChat(`Playing TTS message: ${message}`);
});
