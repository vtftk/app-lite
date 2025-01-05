// Use built-in module for dates
const dateFns = module("date-fns");

// Get the current user and target user from the command context
const { user, targetUserValid } = ctx;

// Name and ID of the person to find
let userName;
let userId;

if (targetUserValid !== null) {
  // Target person was provided as the first argument (e.g !followage user)
  const twitchUser = await api.twitch.getUserByUsername(targetUserValid);

  userId = twitchUser.id;
  userName = twitchUser.name;
} else {
  // Nobody was provided so we use the user who triggered the command
  userId = user.id;
  userName = user.name;
}

// Get the follwer details from twitch
const follower = await api.twitch.getFollower(userId);

// Person is not following
if (follower === null) {
  return `The user ${userName} is not following`;
}

// Convert the date range into a duration
const duration = dateFns.intervalToDuration({
  // Duration started when the user followed
  start: follower.followedAt,
  // Duration ends now
  end: new Date(),
})

// Format the duration (e.g 10 months 18 days 23 hours 30 minutes 55 seconds)
const formattedDuration = dateFns.formatDuration(duration);

return `${userName} has been following for ${formattedDuration}`;
