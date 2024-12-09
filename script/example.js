on("chat", (event) => {
  console.log("Got chat message", event.message, event.user.name);
});
