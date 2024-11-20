import { tweened, type Tweened } from "svelte/motion";
import { writable, type Writable } from "svelte/store";

export type Uuid = string;

export enum ClientMessageType {}

export type ClientMessage = {};

export enum ServerMessageType {
  AuthStateChange = "AuthStateChange",
}

export enum AuthStateChange {
  Authenticated = "Authenticated",
  NotAuthenticated = "NotAuthenticated",
}

export type ServerMessageAuthStateChange = {
  state: AuthStateChange;
};

export type ServerMessage =
  | { type: ServerMessageType.AuthStateChange } & ServerMessageAuthStateChange;

export const authState: Writable<boolean> = writable(false);

let socketStore: WebSocket | null = null;

function createWebsocket(): WebSocket {
  const socket = new WebSocket("ws://localhost:58371/ws");
  socket.onmessage = (ev: MessageEvent) => {
    handleSocketMessage(socket, ev);
  };

  socket.onclose = () => {
    socketStore = null;
  };

  socket.onopen = () => {};

  return socket;
}

try {
  socketStore = createWebsocket();
} catch (e) {
  console.log("failed to create socket", e);
}

async function sendSocketMessage(msg: ClientMessage) {
  if (socketStore === null) return;
  const data = JSON.stringify(msg);
  await socketStore.send(data);
}

function handleAuthStateChange(msg: ServerMessageAuthStateChange) {
  authState.set(msg.state === AuthStateChange.Authenticated);
}

function handleSocketMessage(
  socket: WebSocket,
  ev: MessageEvent<string | Blob>
) {
  const data = ev.data;

  // Cannot handle blob messages
  if (typeof data !== "string") return;

  try {
    const parsed: ServerMessage = JSON.parse(data);

    switch (parsed.type) {
      case ServerMessageType.AuthStateChange:
        handleAuthStateChange(parsed);
        return;
    }
  } catch (e) {
    console.error("failed to parse message", e);
  }
}
