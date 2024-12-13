const message = ctx.message;

await api.twitch.sendChatAnnouncement(message, "primary");
