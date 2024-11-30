declare global {
  interface RuntimeApi {
    /// Twitch related APIs
    twitch: RuntimeTwitchApi;
    /// HTTP related APIs
    http: RuntimeHttpApi;
    /// ... TODO: Local persistent storage APIs for variables
  }

  interface RuntimeTwitchApi {
    /**
     * Send a chat message to twitch
     *
     * @param message Message to send
     * @returns Promise resolved when the message has sent
     */
    sendChat: (message: string) => Promise<void>;
  }

  interface RuntimeHttpApi {
    /**
     * Perform an HTTP get request
     *
     * @param url The URL to request
     * @returns The response text content
     */
    get: (url: string) => Promise<HttpResponse>;
  }

  interface HttpResponse {
    // Whether the status code is a 2xx status code
    ok: boolean;
    // HTTP status code
    status: number;
    // HTTP response text
    response: string;
  }

  // Global API access
  export const api: RuntimeApi;

  type EventMap = {
    chat: ChatEvent;
  };

  interface ChatEvent {
    message: string;
  }

  /**
   * Subscribes to an event
   *
   * @param key Name of the event to subscript to
   * @param callback Callback to run when the event is triggered
   */
  export function on<K extends keyof EventMap>(
    key: K,
    callback: (event: EventMap[K]) => void
  ): void;
}
