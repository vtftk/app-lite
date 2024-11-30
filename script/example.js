on("chat", async (event) => {
  const message = event.message;

  const response = await api.http.get(
    "https://jsonplaceholder.typicode.com/todos/1"
  );

  // Failed to get a response
  if (!response.ok) {
    await api.twitch.sendChat("Failed to retrieve response from API");
    return;
  }

  const data = JSON.parse(response.response);
  const title = data.title;

  await api.twitch.sendChat(
    `You said: "${message}"; Heres a random title: ${title}`
  );
});
