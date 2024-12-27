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
    /// Interact with VTFTK itself
    vtftk: VTFTKApi;
  }

  // Global API access
  export const api: RuntimeApi;

  export type TwitchAnnouncementColor =
    | "blue"
    | "green"
    | "orange"
    | "purple"
    | "primary";

  type UserId = string;

  export interface TwitchUser {
    id: UserId;
    name: string;
    displayName: string;
    profileImageUrl: string;
  }

  export interface RuntimeTwitchApi {
    /**
     * Send a chat message to twitch
     *
     * @param message Message to send
     * @returns Promise resolved when the message has sent
     */
    sendChat: (message: string) => Promise<void>;

    sendChatAnnouncement: (
      message: string,
      color?: TwitchAnnouncementColor,
    ) => Promise<void>;

    getUserByUsername: (username: string) => Promise<TwitchUser>;

    sendShoutout: (targetUserId: UserId) => Promise<void>;

    /**
     * Checks if the user is a vip on the twitch channel
     *
     * @param userId The ID of the user
     * @returns Promise resolved with whether the user is a vip
     */
    isVip: (userId: UserId) => Promise<boolean>;

    /**
     * Checks if the user is a mod on the twitch channel
     *
     * @param userId The ID of the user
     * @returns Promise resolved with whether the user is a mod
     */
    isModerator: (userId: UserId) => Promise<boolean>;

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
      validate?: boolean,
    ) => string | null;
  }

  type HttpMethod =
    | "GET"
    | "POST"
    | "PUT"
    | "DELETE"
    | "PATCH"
    | "OPTIONS"
    | "HEAD"
    | "TRACE"
    | "CONNECT";

  type HttpResponseFormatMap = {
    json: object;
    text: string;
  };

  type HttpResponseFormat = keyof HttpResponseFormatMap;

  type HttpBody = object | string;

  type HttpOptions = Partial<{
    url: string;

    /// HTTP request method
    method: HttpMethod;

    // Response type format expected
    responseFormat: HttpResponseFormat;

    body: HttpBody;

    /// Optional request timeout in milliseconds
    timeout: number;
  }>;

  type HttpResponse<O extends HttpOptions> = {
    // Response status code
    status: number;

    // Response headers
    headers: Partial<Record<string, string>>;

    // Helper to check if the response is a 2xx response code
    get ok(): boolean;

    // Response body
    body: O extends { responseFormat: keyof HttpResponseFormatMap }
      ? HttpResponseFormatMap[O["responseFormat"]]
      : HttpResponseFormatMap["text"];
  };

  export interface RuntimeHttpApi {
    request<O extends HttpOptions>(options: O): Promise<HttpResponse<O>>;

    get<O extends Omit<HttpOptions, "body">>(
      url: string,
      options?: O,
    ): Promise<HttpResponse<O>>;

    post<O extends HttpOptions>(
      url: string,
      body?: HttpBody,
      options?: O,
    ): Promise<HttpResponse<O>>;

    put<O extends HttpOptions>(
      url: string,
      body?: HttpBody,
      options?: O,
    ): Promise<HttpResponse<O>>;

    patch<O extends HttpOptions>(
      url: string,
      body?: HttpBody,
      options?: O,
    ): Promise<HttpResponse<O>>;

    delete<O extends HttpOptions>(
      url: string,
      body?: HttpBody,
      options?: O,
    ): Promise<HttpResponse<O>>;
  }

  export interface KvStoreApi {
    getRaw: (key: string) => Promise<string | null>;
    remove: (key: string) => Promise<void>;

    getText: (key: string, defaultValue?: string) => Promise<string | null>;
    setText: (key: string, value: string) => Promise<void>;

    getNumber: (key: string, defaultValue?: number) => Promise<number | null>;
    setNumber: (key: string, value: number) => Promise<number>;

    getArray: (key: string, defaultValue?: any[]) => Promise<any[] | null>;
    setArray: (key: string, value: any[]) => Promise<any[]>;

    getObject: (key: string, defaultValue?: any) => Promise<any | null>;
    setObject: (key: string, value: any) => Promise<void>;

    createCounter: (key: string) => KvCounter;
    createScopedCounter: (key: string) => KvScopedCounter;
  }

  export interface KvCounter {
    /**
     * Get the current counter value
     *
     * @returns Promise resolved to the current counter value
     */
    get: () => Promise<number>;

    /**
     * Set the counter to a specific value
     *
     * @param value The value to set the counter to
     * @returns Promise resolved when the counter is updated
     */
    set: (value: number) => Promise<void>;

    /**
     * Increase the counter by the provided amount, defaults to 1
     *
     * @param amount Amount to increase by (Defaults to 1)
     * @returns Promise resolved to the new counter value
     */
    increase: (amount?: number) => Promise<number>;

    /**
     * Decrease the counter by the provided amount, defaults to 1
     *
     * @param amount Amount to increase by (Defaults to 1)
     * @returns Promise resolved to the new counter value
     */
    decrease: (amount?: number) => Promise<number>;
  }

  export interface KvScopedCounter {
    get: (scope: string) => Promise<number>;
    set: (scope: string, value: number) => Promise<void>;
    increase: (scope: string, amount?: number) => Promise<number>;
    decrease: (scope: string, amount?: number) => Promise<number>;
    all: () => Promise<ScopedCounterEntry[]>;
  }

  export interface ScopedCounterEntry {
    scope: string;
    amount: number;
  }

  interface TTSMonsterVoice {
    voice_id: string;
    name: string;
    sample: string;
  }

  interface SoundSeq {
    src: string;
    volume: number;
  }

  export interface VTFTKApi {
    ttsVoices: () => Promise<TTSMonsterVoice[]>;

    ttsGenerate: (voice_id: string, message: string) => Promise<string>;
    ttsGenerateParsed: (message: string) => Promise<string[]>;

    /**
     * Play a sound through the overlay
     *
     * @param src URL of the sound to play
     * @param volume Volume to play the sound at 0-1
     * @returns
     */
    playSound: (src: string, volume?: number) => Promise<void>;
    playSoundSeq: (sounds: SoundSeq) => Promise<void>;
  }

  export interface TwitchEventUser {
    id: string;
    name: string;
    display_name: string;
  }

  type SubscriptionTier = "1000" | "2000" | "3000" | "Prime" | string;

  export type EventData = {
    user: TwitchEventUser;
  };

  export type EventInputData = {
    redeem: {
      reward_name: string;
      reward_id: string;
      cost: number;
      user_input: string;
    };
    cheerBits: {
      bits: number;
      anonymous: boolean;
      message: string;
    };
    follow: {};
    subscription: {
      tier: SubscriptionTier;
      is_gift: boolean;
    };
    giftSubscription: {
      tier: SubscriptionTier;
      cumulative_total: number | null;
      anonymous: boolean;
    };
    reSubscription: {
      cumulative_months: number;
      duration_months: number;
      message: string;
      streak_months: number | null;
      tier: SubscriptionTier;
    };
    chat: {
      message: string;
      fragments: any[];
      cheer: number | null;
    };
    raid: {
      viewers: number;
    };
  };

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

    /**
     * Get the target user of the command within the context of a command
     * only available within command scripts
     *
     * Helper for `api.twitch.getUsernameArg(ctx.args[0], false)`
     *
     * The twitch name or null if its invalid or missing
     */
    get targetUser(): string | null;

    /**
     * Get the target user of the command within the context of a command
     * only available within command scripts
     *
     * Helper for `api.twitch.getUsernameArg(ctx.args[0], true)`
     *
     * The twitch name or null if its invalid or missing. Performs extra validation
     * to ensure the argument is actually a valid username
     */
    get targetUserValid(): string | null;
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
  const ctx: CommandContext;

  /**
   * Get the target user of the command within the context of a command
   * only available within command scripts
   *
   * Helper for `api.twitch.getUsernameArg(ctx.args[0], validate)`
   *
   * @param validate Whether to validate if the name is a valid twitch name
   * @return The twitch name or null if its invalid or missing
   */
  export function getTargetUser(validate: boolean): string | null;

  /**
   * Subscribes to an event
   *
   * @param key Name of the event to subscript to
   * @param callback Callback to run when the event is triggered
   */
  export function on<K extends keyof EventInputData>(
    key: K,
    callback: (event: EventData & EventInputData[K]) => void,
  ): void;
}
