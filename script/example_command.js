const message = ctx.message;

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

const randomNumber = Math.floor(Math.random() * 100000);
return `You said: "${message}"; Heres a random title: ${title}, Heres a random number: ${randomNumber}`;
