/// <reference no-default-lib="true" />
/**
 * Runs the provided function within the specific logging context
 *
 * @param ctx The logging context
 * @param callback The function to run
 * @param args Arguments for the function
 */
declare function runWithContext<C, A extends any[], R>(ctx: C, callback: (...args: A) => R, ...args: A): R;
/**
 * Get the current logging context set by {@see runWithContext}
 *
 * @returns The current context or undefined if not within a context
 */
declare function getContext<T>(): T;
/**
 * Log the provided arguments at the "INFO" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
declare function info(...args: unknown[]): void;
/**
 * Log the provided arguments at the "ERROR" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
declare function error(...args: unknown[]): void;
/**
 * Log the provided arguments at the "WARN" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
declare function warn(...args: unknown[]): void;
/**
 * Log the provided arguments at the "DEBUG" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
declare function debug(...args: unknown[]): void;

declare const logging_debug: typeof debug;
declare const logging_error: typeof error;
declare const logging_getContext: typeof getContext;
declare const logging_info: typeof info;
declare const logging_runWithContext: typeof runWithContext;
declare const logging_warn: typeof warn;
declare namespace logging {
  export { logging_debug as debug, logging_error as error, logging_getContext as getContext, logging_info as info, logging_runWithContext as runWithContext, logging_warn as warn };
}

/**
 * Store a string value within the KV store
 *
 * @param key The key to store the value under
 * @param value The string value to store
 * @returns Promise resolved when the value is stored
 */
declare function setText(key: string, value: string): Promise<void>;
/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
declare function getText(key: string, defaultValue: string): Promise<string>;
/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
declare function getText(key: string): Promise<string | null>;
/**
 * Remove a key value pair from the KV store
 *
 * @param key The key to remove
 * @returns Promise resolved when the value is removed
 */
declare function remove(key: string): Promise<void>;
/**
 * Store a number value within the KV store
 *
 * @param key The key to store the value under
 * @param value The number value to store
 * @returns Promise resolved when the value is stored
 */
declare function setNumber(key: string, value: number): Promise<void>;
/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
declare function getNumber(key: string, defaultValue: number): Promise<number>;
/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
declare function getNumber(key: string): Promise<number | null>;
/**
 * Store an array value within the KV store
 *
 * @param key The key to store the value under
 * @param value The array value to store
 * @returns Promise resolved when the value is stored
 */
declare function setArray<T>(key: string, value: T[]): Promise<void>;
/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
declare function getArray<T>(key: string, defaultValue: T[]): Promise<T[]>;
/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
declare function getArray<T>(key: string): Promise<T[] | null>;
/**
 * Store an object value within the KV store
 *
 * @param key The key to store the value under
 * @param value The object value to store
 * @returns Promise resolved when the value is stored
 */
declare function setObject<T>(key: string, value: T): Promise<void>;
/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored
 * @returns Promise resolved to the value, will be the defaultValue if nothing is stored
 */
declare function getObject<T>(key: string, defaultValue: T): Promise<T>;
/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @returns Promise resolved to the value, will be null if no value is stored
 */
declare function getObject<T>(key: string): Promise<T | null>;
interface Counter {
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
/**
 * Create a new counter using the provided key
 *
 * @param key The key to store the counter value within
 * @returns The created counter
 */
declare function createCounter(key: string): Counter;
type ScopedCounterObject = Partial<Record<string, number>>;
interface ScopedCounterEntry {
    scope: string;
    amount: number;
}
interface ScopedCounter {
    /**
     * Get the counter value for the provided scope
     *
     * @param scope The scope to get the counter for (i.e the user name)
     * @returns Promise resolved to the scope value
     */
    get(scope: string): Promise<number>;
    /**
     * Set the counter value for a specific scope
     *
     * @param scope The scope to get the counter for (i.e the user name)
     * @param value The value to set for the scope
     * @returns Promise resolved when the value is updated
     */
    set(scope: string, value: number): Promise<void>;
    /**
     * Increase the counter value for a specific scope
     *
     * @param scope The scope to get the counter for (i.e the user name)
     * @param amount Amount to increase the counter by (Default: 1)
     * @returns Promise resolved when the value is updated returns the new value after the update
     */
    increase(scope: string, amount?: number): Promise<number>;
    /**
     * Decrease the counter value for a specific scope
     *
     * @param scope The scope to get the counter for (i.e the user name)
     * @param amount Amount to decrease the counter by (Default: 1)
     * @returns Promise resolved when the value is updated returns the new value after the update
     */
    decrease(scope: string, amount?: number): Promise<number>;
    /**
     * Gets all entries within the scoped counter
     *
     * @returns Promise resolved to the list of entries
     */
    all(): Promise<ScopedCounterEntry[]>;
}
/**
 * Create a new scoped counter using the provided key
 *
 * Scoped counters provide a way to track a counter for a specific "scope"
 * this can be used to create per-user counters or per-game counters
 *
 * @param key The key to store the counter value within
 * @returns The created scoped counter
 */
declare function createScopedCounter(key: string): ScopedCounter;

type kv_Counter = Counter;
type kv_ScopedCounter = ScopedCounter;
type kv_ScopedCounterEntry = ScopedCounterEntry;
type kv_ScopedCounterObject = ScopedCounterObject;
declare const kv_createCounter: typeof createCounter;
declare const kv_createScopedCounter: typeof createScopedCounter;
declare const kv_getArray: typeof getArray;
declare const kv_getNumber: typeof getNumber;
declare const kv_getObject: typeof getObject;
declare const kv_getText: typeof getText;
declare const kv_remove: typeof remove;
declare const kv_setArray: typeof setArray;
declare const kv_setNumber: typeof setNumber;
declare const kv_setObject: typeof setObject;
declare const kv_setText: typeof setText;
declare namespace kv {
  export { type kv_Counter as Counter, type kv_ScopedCounter as ScopedCounter, type kv_ScopedCounterEntry as ScopedCounterEntry, type kv_ScopedCounterObject as ScopedCounterObject, kv_createCounter as createCounter, kv_createScopedCounter as createScopedCounter, kv_getArray as getArray, kv_getNumber as getNumber, kv_getObject as getObject, kv_getText as getText, kv_remove as remove, kv_setArray as setArray, kv_setNumber as setNumber, kv_setObject as setObject, kv_setText as setText };
}

/**
 * Send a chat message to twitch
 *
 * @param message Message to send
 * @returns Promise resolved when the message has sent
 */
declare function sendChat(message: string): Promise<void>;
type TwitchAnnouncementColor = "blue" | "green" | "orange" | "purple" | "primary";
/**
 * Send a twitch chat announcement
 *
 * @param message Message to send
 * @param color Optional message color (Defaults to primary color)
 * @returns Promise resolved when the message has sent
 */
declare function sendChatAnnouncement(message: string, color?: TwitchAnnouncementColor): Promise<void>;
type TwitchUserId = string;
type TwitchUsername = string;
interface TwitchUser {
    id: TwitchUserId;
    name: string;
    displayName: string;
    profileImageUrl: string;
}
/**
 * Attempts to lookup a twitch user by username
 *
 * @param username Username of the user to get
 * @returns Promise resolved to the twitch user
 */
declare function getUserByUsername(username: TwitchUsername): Promise<TwitchUser>;
/**
 * Triggers a twitch shoutout for the provided use
 *
 * @param userId The ID of the user to shoutout
 * @returns Promise resolved when the shoutout is complete
 */
declare function shoutout(userId: TwitchUserId): Promise<void>;
/**
 * Checks if the user is a mod on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a mod
 */
declare function isModerator(userId: TwitchUserId): Promise<void>;
/**
 * Checks if the user is a vip on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a vip
 */
declare function isVip(userId: TwitchUserId): Promise<void>;
interface TwitchFollower {
    id: TwitchUserId;
    name: TwitchUsername;
    displayName: string;
    followedAt: Date;
}
/**
 * Gets a twitch follower by ID
 *
 * Can be used to get the followedAt timestamp for when
 * the user followed the broadcaster
 *
 * @param userId The twitch user ID to get the follower for
 * @returns The follower or null if the user is not following
 */
declare function getFollower(userId: TwitchUserId): Promise<TwitchFollower | null>;
/**
 * Attempts to extract a username from the provided arg
 *
 * Normalizes the username into a format without @ and without
 * any leading or trailing whitespace, optionally validating
 * that the username is a valid twitch username
 *
 * @param rawArg Raw argument to attempt to get a username from
 * @param validate Whether the validate the username
 * @returns The username or null if the username is invalid or missing
 */
declare function getUsernameArg(rawArg: unknown, validate?: boolean): string | null;
/**
 * Applies strict validation against the provided username
 * to ensure that it is a twitch username ensuring the correct
 * character and length requirements
 *
 * @param username The username to check
 * @returns Whether the username is valid
 */
declare function isValidUsernameStrict(username: TwitchUsername): boolean;

type twitch_TwitchAnnouncementColor = TwitchAnnouncementColor;
type twitch_TwitchFollower = TwitchFollower;
type twitch_TwitchUser = TwitchUser;
type twitch_TwitchUserId = TwitchUserId;
type twitch_TwitchUsername = TwitchUsername;
declare const twitch_getFollower: typeof getFollower;
declare const twitch_getUserByUsername: typeof getUserByUsername;
declare const twitch_getUsernameArg: typeof getUsernameArg;
declare const twitch_isModerator: typeof isModerator;
declare const twitch_isValidUsernameStrict: typeof isValidUsernameStrict;
declare const twitch_isVip: typeof isVip;
declare const twitch_sendChat: typeof sendChat;
declare const twitch_sendChatAnnouncement: typeof sendChatAnnouncement;
declare const twitch_shoutout: typeof shoutout;
declare namespace twitch {
  export { type twitch_TwitchAnnouncementColor as TwitchAnnouncementColor, type twitch_TwitchFollower as TwitchFollower, type twitch_TwitchUser as TwitchUser, type twitch_TwitchUserId as TwitchUserId, type twitch_TwitchUsername as TwitchUsername, twitch_getFollower as getFollower, twitch_getUserByUsername as getUserByUsername, twitch_getUsernameArg as getUsernameArg, twitch_isModerator as isModerator, twitch_isValidUsernameStrict as isValidUsernameStrict, twitch_isVip as isVip, twitch_sendChat as sendChat, twitch_sendChatAnnouncement as sendChatAnnouncement, twitch_shoutout as shoutout };
}

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "OPTIONS" | "HEAD" | "TRACE" | "CONNECT";
type HttpResponseFormatMap = {
    json: object;
    text: string;
};
type HttpResponseFormat = keyof HttpResponseFormatMap;
type HttpBody = object | string;
type HttpOptions = Partial<{
    url: string;
    method: HttpMethod;
    responseFormat: HttpResponseFormat;
    headers: Partial<Record<string, string>>;
    body: HttpBody;
    timeout: number;
}>;
type HttpResponse<BodyFormat> = {
    status: number;
    headers: Partial<Record<string, string>>;
    get ok(): boolean;
    body: HttpResponseBody<BodyFormat>;
};
type HttpResponseBody<F> = F extends keyof HttpResponseFormatMap ? HttpResponseFormatMap[F] : HttpResponseFormatMap["text"];
declare function request<O extends HttpOptions>(options: O): Promise<HttpResponse<O["responseFormat"]>>;
type GetHttpOptions = Omit<HttpOptions, "body" | "method" | "url">;
declare function get<O extends GetHttpOptions>(url: string, options?: O): Promise<HttpResponse<O["responseFormat"]>>;
declare function post<B extends HttpBody | undefined, O extends HttpOptions>(url: string, body?: B, options?: O): Promise<HttpResponse<O["responseFormat"]>>;
declare function put<B extends HttpBody | undefined, O extends HttpOptions>(url: string, body?: B, options?: O): Promise<HttpResponse<O["responseFormat"]>>;
declare function patch<B extends HttpBody | undefined, O extends HttpOptions>(url: string, body?: B, options?: O): Promise<HttpResponse<O["responseFormat"]>>;

declare const http_get: typeof get;
declare const http_patch: typeof patch;
declare const http_post: typeof post;
declare const http_put: typeof put;
declare const http_request: typeof request;
declare namespace http {
  export { http_get as get, http_patch as patch, http_post as post, http_put as put, http_request as request };
}

type TTSMonsterVoiceId = string;
interface TTSMonsterVoice {
    voice_id: TTSMonsterVoiceId;
    name: string;
    sample: string;
}
/**
 * Requests the list of voices from TTS Monster
 *
 * @returns The list of available voices
 */
declare function voices(): Promise<TTSMonsterVoice[]>;
/**
 * Generate a single voice message using a specific voice
 *
 * @param voice_id The ID of the voice to use
 * @param message The message for the voice to say
 * @returns URL to the voice message file
 */
declare function generate(voice_id: TTSMonsterVoiceId, message: string): Promise<string>;
/**
 * Generates a TTS voices uses the names and messages parsed from the
 * provided message i.e
 *
 *  "(Name1) This is the message for Name1 (Name2) This is the message for Name2"
 *
 * This will create voice messages for each of the voices returning the
 * messages in order
 *
 * @param message The message to parse and generate
 * @returns The list of URLs for each voice message segment
 */
declare function generateParsed(message: string): Promise<string[]>;

type tts_monster_d_TTSMonsterVoice = TTSMonsterVoice;
type tts_monster_d_TTSMonsterVoiceId = TTSMonsterVoiceId;
declare const tts_monster_d_generate: typeof generate;
declare const tts_monster_d_generateParsed: typeof generateParsed;
declare const tts_monster_d_voices: typeof voices;
declare namespace tts_monster_d {
  export { type tts_monster_d_TTSMonsterVoice as TTSMonsterVoice, type tts_monster_d_TTSMonsterVoiceId as TTSMonsterVoiceId, tts_monster_d_generate as generate, tts_monster_d_generateParsed as generateParsed, tts_monster_d_voices as voices };
}

declare namespace integrations {
  export { tts_monster_d as tts_monster };
}

/**
 * Plays the provided sound through the overlay
 *
 * If you are playing multiple sounds that need to be triggered
 * one after the other use {@see playSoundSeq} instead, play sound
 * promise completes after its been queued not after its finished
 * playing
 *
 * @param src The src URL for the sound file
 * @param volume The volume to play the sound at
 * @returns Promise resolved when the sound has been sent to the event queue
 */
declare function playSound(src: string, volume?: number): Promise<void>;
interface SoundSeq {
    src: string;
    volume: number;
}
/**
 * Plays the provided collection of sound through the overlay
 * one by one, only starts playing the next sound after the
 * first sound completes
 *
 * @param sounds Sequence of sounds to play
 * @returns Promise resolved when the sounds has been sent to the event queue
 */
declare function playSoundSeq(sounds: SoundSeq[]): Promise<void>;

declare const vtftk_playSound: typeof playSound;
declare const vtftk_playSoundSeq: typeof playSoundSeq;
declare namespace vtftk {
  export { vtftk_playSound as playSound, vtftk_playSoundSeq as playSoundSeq };
}

/// <reference no-default-lib="true" />



declare global {
  export interface Api {
    /// Twitch related APIs
    twitch: typeof twitch;
    /// HTTP related APIs
    http: typeof http;
    /// Logging related APIs
    logging: typeof logging;
    /// Key value store
    kv: typeof kv;
    /// Interact with VTFTK itself
    vtftk: typeof vtftk;
    // External integrations such as TTS Monster
    integrations: typeof integrations;
  }

  // Array prototype extensions
  interface Array<T> {
    random(): T;
  }

  // Global API access
  export const api: Api;
}


declare global {
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

  export type EventInputValue = EventInputData[keyof EventInputData];

  /**
   * Event data, only available within the context of a event outcome script
   */
  const event: EventData & EventInputValue;
}

declare global {
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
}
export {};