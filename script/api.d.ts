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

    /**
     * Validates a Twitch username, strict checks ensuring the name
     * meets the required length and allowed characters
     *
     * @param username The username to validate
     * @returns Whether the username is valid
     */
    isValidUsernameStrict: (username: string) => boolean;

    /**
     * Attempts to extract a username from the provided arg
     *
     * @param arg The arg to get the username from
     * @param valid When enabled the value is also checked against {@see isValidUsernameStrict}
     * @returns The arg or null if missing/invalid
     */
    getUsernameArg: (
      arg: string | undefined | null,
      validate: boolean = false
    ) => string | null;
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
     * Target user, will be present if a first argument is
     * provided
     */
    targetUser: string | null;

    /**
     * Message split into the individual arguments split by space.
     * Excludes the command itself
     */
    args: string[];
  }

  export type CommandUser = {
    id: string;
    name: string;
    displayName: string;
  };

  /**
   * Context for the current command execution, only available within
   * command scripts
   */
  declare const ctx: CommandContext;

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
