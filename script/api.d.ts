export {};

declare global {
  // Type of function used for logging
  type LogFunction = (...arg: any) => void;

  export interface LoggingApi {
    debug: LogFunction;
    info: LogFunction;
    warn: LogFunction;
    error: LogFunction;
  }

  export interface RuntimeApi {
    /// Twitch related APIs
    twitch: RuntimeTwitchApi;
    /// HTTP related APIs
    http: RuntimeHttpApi;
    /// Logging related APIs
    logging: LoggingApi;
    /// Key value store
    kv: KvStoreApi;
  }

  // Global API access
  export const api: RuntimeApi;

  export interface RuntimeTwitchApi {
    /**
     * Send a chat message to twitch
     *
     * @param message Message to send
     * @returns Promise resolved when the message has sent
     */
    sendChat: (message: string) => Promise<void>;

    /**
     * Checks if the user is a vip on the twitch channel
     *
     * @param userId The ID of the user
     * @returns Promise resolved with whether the user is a vip
     */
    isVip: (userId: string) => Promise<boolean>;

    /**
     * Checks if the user is a mod on the twitch channel
     *
     * @param userId The ID of the user
     * @returns Promise resolved with whether the user is a mod
     */
    isModerator: (userId: string) => Promise<boolean>;
  }

  export interface RuntimeHttpApi {
    /**
     * Perform an HTTP get request
     *
     * @param url The URL to request
     * @returns The response text content
     */
    get: (url: string) => Promise<HttpResponse>;
  }

  export interface KvStoreApi {
    get: (key: string) => Promise<string | null>;
    remove: (key: string) => Promise<void>;
    set: (key: string, value: string) => Promise<void>;
  }

  export interface HttpResponse {
    // Whether the status code is a 2xx status code
    ok: boolean;
    // HTTP status code
    status: number;
    // HTTP response text
    response: string;
  }

  export type EventMap = {
    chat: ChatEvent;
  };

  export interface ChatEvent {
    user_id: string;
    user_name: string;
    user_display_name: string;
    message: string;
  }

  export interface CommandContext {
    // Full original message
    fullMessage: string;

    // Message with the command prefix stripped
    message: string;

    // User who executed the command
    user: CommandUser;

    /**
     * Message split into the individual arguments split by space.
     * Excludes the command itself
     */
    args: string[];
  }

  export type CommandUser = {
    id: string;
    name: string;
    display_name: string;
  };

  interface CreateCommandOptions {
    // Command required to trigger, including the command prefix (e.g !test)
    command: string;

    requireVip?: boolean;
    requireMod?: boolean;

    /**
     * Handle a command using this function
     *
     * If the return value is a string that will be sent
     * as a chat message
     *
     * @param ctx Context around the command
     * @returns
     */
    handle: (
      ctx: CommandContext
    ) => string | Promise<string> | any | Promise<any>;
  }

  export function createCommand(options: CreateCommandOptions);

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
