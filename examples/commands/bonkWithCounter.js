const { user, args } = ctx;
const targetUser = api.twitch.getUsernameArg(args[0]);

// Cannot hug nobody
if (targetUser === null) {
  return "....You can't bonk nobody silly... you gotta put the name";
}

// Strip any extra parts from the name
const targetUserClean = targetUser.toLowerCase();

// Create bonking counter
const bonkCounters = api.kv.createScopedCounter("bonkCounters");

// Increase the bonk counter
const userBonks = await bonkCounters.increase(targetUserClean, 1);

return `-----|_| ${user.name} has bonked ${targetUser}. ${targetUser} has been bonked ${userBonks} times.`;
