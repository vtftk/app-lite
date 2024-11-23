import { attemptAuthorization } from "./auth";
import { APIError } from "./error";
import { VTubeMessage } from "./message";

type PromiseExecutor = {
  resolve: (value: VTubeMessage<any>) => void;
  reject: (reason?: any) => void;
};

const RETRY_TIMEOUT = 5 * 1000;

export class VTubeStudioWebSocket {
  private host: string;
  private port: number;

  private websocket: WebSocket | null;
  private requestID: number;
  private requestHandlers: Map<number, PromiseExecutor>;

  onDisconnect: VoidFunction | undefined;
  onConnected: VoidFunction | undefined;

  constructor(host: string, port: number) {
    this.host = host;
    this.port = port;
    this.websocket = null;
    this.requestID = 0;
    this.requestHandlers = new Map();
  }

  connect() {
    const tryConnect = () => {
      this.websocket = new WebSocket(`ws://${this.host}:${this.port}/`);

      this.websocket.onopen = async () => {
        const socket = this.websocket!;
        if (socket.readyState !== WebSocket.OPEN) return;

        await attemptAuthorization(this);

        console.debug("VTube studio authorization complete");

        if (this.onConnected) this.onConnected();
      };

      this.websocket.onmessage = this.handleSocketMessage.bind(this);
      this.websocket.onclose = (event: CloseEvent) => {
        if (this.onDisconnect) this.onDisconnect();

        console.warn(
          "VTube studio WebSocket closed:",
          event.code,
          event.reason
        );

        this.clearRequestHandlers();
        this.websocket = null;

        setTimeout(() => {
          console.log(`Reconnecting...`);
          tryConnect();
        }, RETRY_TIMEOUT);
      };

      this.websocket.onerror = (error: Event) => {
        console.error("VTube studio WebSocket error:", error);
        this.websocket?.close();
      };
    };

    tryConnect();
  }

  handleSocketMessage(message: MessageEvent<string>) {
    const response: VTubeMessage<any> = JSON.parse(message.data);

    // Response didn't belong to any handler
    if (!response.requestID) {
      console.debug("request without identifier", response);
      return;
    }

    const requestID = Number(response.requestID);

    // We don't have a message handler available
    const handler = this.requestHandlers.get(requestID);
    if (handler === undefined) {
      console.debug("response for undefined handler", response);
      return;
    }

    this.requestHandlers.delete(requestID);

    if (response.messageType === "APIError") {
      // Handle API errors
      handler.reject(
        new APIError(response.data.errorID, response.data.message)
      );
    } else {
      handler.resolve(response);
    }
  }

  send(msg: VTubeMessage<any>): Promise<VTubeMessage<any>> {
    return new Promise((resolve, reject) => {
      if (
        this.websocket === null ||
        this.websocket.readyState !== WebSocket.OPEN
      ) {
        return reject(new Error("socket is not currently open"));
      }

      // Update request ID
      let msgRequestID = this.requestID++;
      msg.requestID = "" + msgRequestID;

      const data = JSON.stringify(msg);

      this.requestHandlers.set(msgRequestID, { resolve, reject });
      this.websocket.send(data);
    });
  }

  close() {
    if (this.websocket) {
      this.websocket.close();
    }
  }

  clearRequestHandlers() {
    this.requestHandlers.forEach((executor) => executor.reject());
    this.requestHandlers.clear();
  }
}
