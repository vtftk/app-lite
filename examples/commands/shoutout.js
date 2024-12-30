const { targetUser } = ctx;

let twitchUser;

// find the user to shoutout
try {
  twitchUser = await api.twitch.getUserByUsername(targetUser);

  if (twitchUser === null) {
    return "unable to find that twitch user, check the username is correct";
  }
} catch (e) {
  console.error("failed to get twitch user", e);
  return "failed to find user, check that the username is correct";
}

// Send shoutout
try {
  await api.twitch.shoutout(twitchUser.id);
} catch (e) {
  // Ignored.. official shoutout can fail due to ratelimiting or if the user isn't streaming
  console.error("failed to send shoutout", e);
}

return `Go check out ${twitchUser.displayName} on twitch at https://twitch.tv/${twitchUser.name}`;
