const { user, targetUserValid } = ctx;

let userName = user.name;
let userId = user.id;

if (targetUserValid !== null) {
  // Find the twitch user
  const twitchUser = await api.twitch.getUserByUsername(targetUser);

  userId = twitchUser.id;
  userName = twitchUser.name;
}

const follower = await api.twitch.getFollower(userId);

if (follower === null) {
  return `The user ${userName} is not following`;
}

const startDate = new Date(follower.followed_at);
const endDate = new Date();

let years = endDate.getFullYear() - startDate.getFullYear();
let months = endDate.getMonth() - startDate.getMonth();
let days = endDate.getDate() - startDate.getDate();
let hours = endDate.getHours() - startDate.getHours();

// Adjust months and years if necessary
if (days < 0) {
  months -= 1;
  const previousMonth = new Date(endDate.getFullYear(), endDate.getMonth(), 0);
  days += previousMonth.getDate();
}

if (months < 0) {
  years -= 1;
  months += 12;
}

// Build the result string
const parts = [];
if (years > 0) parts.push(`${years} year${years > 1 ? "s" : ""}`);
if (months > 0) parts.push(`${months} month${months > 1 ? "s" : ""}`);
if (days > 0) parts.push(`${days} day${days > 1 ? "s" : ""}`);
if (hours > 0) parts.push(`${hours} hours${hours > 1 ? "s" : ""}`);

const dateDistance = parts.length > 0 ? parts.join(", ") : "0 hours";

return `${userName} has been following for ${dateDistance}`;
