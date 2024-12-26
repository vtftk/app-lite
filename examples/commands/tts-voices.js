const message = ctx.message;

// Acknowledge the request
api.twitch.sendChat("Loading available voices.....");

// Get list of voices
const voices = await api.vtftk.ttsVoices(message);

await api.twitch.sendChat("Available voices: ");

let voicesMessage = "";

for (const voice of voices) {
  // Send message if it gets too long
  if (voice.name.length + voicesMessage.length > 400) {
    await api.twitch.sendChat(voicesMessage);
    voicesMessage = "";
  }

  if (voicesMessage.length > 0) voicesMessage += ", ";
  voicesMessage += voice.name;
}

// Send left over chunk of messages
if (voicesMessage.length > 0) {
  await api.twitch.sendChat(voicesMessage + ".");
}
