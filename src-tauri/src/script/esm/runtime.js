const { AsyncVariable, setAsyncContext } = Deno.core;
// Async variable for storing logging context
const loggingContextVariable = new AsyncVariable();
/**
 * Runs the provided function within the specific logging context
 *
 * @param ctx The logging context
 * @param callback The function to run
 * @param args Arguments for the function
 */
function runWithContext(ctx, callback, ...args) {
    const previous = loggingContextVariable.enter(ctx);
    try {
        return Reflect.apply(callback, null, args);
    }
    finally {
        setAsyncContext(previous);
    }
}
/**
 * Get the current logging context set by {@see runWithContext}
 *
 * @returns The current context or undefined if not within a context
 */
function getContext() {
    return loggingContextVariable.get();
}
/**
 * Log the provided arguments at the "INFO" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
function info(...args) {
    _log("Info", ...args);
}
/**
 * Log the provided arguments at the "ERROR" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
function error(...args) {
    _log("Error", ...args);
}
/**
 * Log the provided arguments at the "WARN" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
function warn(...args) {
    _log("Warn", ...args);
}
/**
 * Log the provided arguments at the "DEBUG" level
 *
 * @param args Arguments to log, can be strings, objects or any other value
 */
function debug(...args) {
    _log("Debug", ...args);
}
/**
 * Internal logging function calls the Deno op to trigger
 * logging on the Rust end
 *
 * @param level The log level
 * @param args Arguments to log
 */
function _log(level, ...args) {
    const ctx = getContext();
    Deno.core.ops.op_log(ctx, level, stringifyArgs(...args));
}
/**
 * Stringify a collection of arguments for logging
 *
 * @param args Arguments to stringify
 * @returns The stringified arguments
 */
function stringifyArgs(...args) {
    return args.map((arg) => stringify(arg)).join(" ");
}
/**
 * Deeply convert a value to a string, handles self referencing
 * objects by replacing them with <ref:{path}>
 *
 * @param data Value to print
 * @returns The string version of the value
 */
function stringify(data) {
    // Handle special cases
    if (data === undefined)
        return "undefined";
    if (data === null)
        return "null";
    if (typeof data === "string")
        return data;
    if (data instanceof Error) {
        return JSON.stringify(data, Object.getOwnPropertyNames(data));
    }
    const seen = [];
    const keys = [];
    function stringify(key, value) {
        // Skip non or null/undefined objects
        if (typeof value !== "object" || !value)
            return value;
        let index = seen.indexOf(value);
        // Have not seen the value yet
        if (index === -1) {
            seen.push(value);
            keys.push(key);
            return value;
        }
        // Build the reference path for previously seen objects
        let topKey = keys[index];
        const path = [topKey];
        // Trace back to find the full path of the circular reference
        for (index--; index > 0; index--) {
            if (seen[index][topKey] === value) {
                value = seen[index];
                topKey = keys[index];
                path.unshift(topKey);
            }
        }
        return "<ref:" + path.join(".") + ">";
    }
    return JSON.stringify(data, stringify, 2);
}

var logging = /*#__PURE__*/Object.freeze({
    __proto__: null,
    debug: debug,
    error: error,
    getContext: getContext,
    info: info,
    runWithContext: runWithContext,
    warn: warn
});

/**
 * Store a string value within the KV store
 *
 * @param key The key to store the value under
 * @param value The string value to store
 * @returns Promise resolved when the value is stored
 */
function setText(key, value) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    if (typeof value !== "string")
        throw new Error("value must be a string");
    return Deno.core.ops.op_kv_set("Text", key, value);
}
/**
 * Get a text value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the text value, null if there is no value and no default is specified
 */
async function getText(key, defaultValue) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null && defaultValue !== undefined)
        return defaultValue;
    return value;
}
/**
 * Remove a key value pair from the KV store
 *
 * @param key The key to remove
 * @returns Promise resolved when the value is removed
 */
function remove(key) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    return Deno.core.ops.op_kv_remove(key);
}
/**
 * Store a number value within the KV store
 *
 * @param key The key to store the value under
 * @param value The number value to store
 * @returns Promise resolved when the value is stored
 */
function setNumber(key, value) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    if (typeof value !== "number")
        throw new Error("value must be a number");
    return Deno.core.ops.op_kv_set("Number", key, value);
}
/**
 * Get a number value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the number value, null if there is no value and no default is specified
 */
async function getNumber(key, defaultValue) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null)
        return defaultValue ?? null;
    return Number(value);
}
/**
 * Store an array value within the KV store
 *
 * @param key The key to store the value under
 * @param value The array value to store
 * @returns Promise resolved when the value is stored
 */
function setArray(key, value) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    if (!Array.isArray(value))
        throw new Error("value must be an array");
    return Deno.core.ops.op_kv_set("Array", key, value);
}
/**
 * Get an array value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the array value, null if there is no value and no default is specified
 */
async function getArray(key, defaultValue) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null)
        return defaultValue ?? null;
    return JSON.parse(value);
}
/**
 * Store an object value within the KV store
 *
 * @param key The key to store the value under
 * @param value The object value to store
 * @returns Promise resolved when the value is stored
 */
function setObject(key, value) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    if (typeof value !== "object")
        throw new Error("value must be a object");
    return Deno.core.ops.op_kv_set("Object", key, JSON.stringify(value));
}
/**
 * Get an object value from the KV store
 *
 * @param key The key the value is under
 * @param defaultValue Default value, used if there is no matching key stored (Default: null)
 * @returns Promise resolved to the object value, null if there is no value and no default is specified
 */
async function getObject(key, defaultValue) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    const value = await Deno.core.ops.op_kv_get(key);
    if (value === null)
        return defaultValue ?? null;
    return JSON.parse(value);
}
/**
 * Create a new counter using the provided key
 *
 * @param key The key to store the counter value within
 * @returns The created counter
 */
function createCounter(key) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    const update = async (action) => {
        const value = await getNumber(key, 0);
        const updated = action(value);
        await setNumber(key, updated);
        return updated;
    };
    return {
        get: () => getNumber(key, 0),
        set: (value) => setNumber(key, value),
        increase: (amount) => update((value) => value + (amount ?? 1)),
        decrease: (amount) => update((value) => value - (amount ?? 1)),
    };
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
function createScopedCounter(key) {
    if (typeof key !== "string")
        throw new Error("key must be a string");
    /**
     * Updates the value at the provided scope returning
     * the new value
     *
     * @param scope The scope to update
     * @param action The action to transform the value
     * @returns Promise resolved to the new value
     */
    const update = async (scope, action) => {
        const objectValue = await getObject(key, {});
        const value = objectValue[scope] ?? 0;
        const updated = action(value);
        objectValue[scope] = updated;
        await setObject(key, objectValue);
        return updated;
    };
    return {
        get: async (scope) => {
            if (typeof scope !== "string")
                throw new Error("scope must be a string");
            const objectValue = await getObject(key, {});
            return objectValue[scope] ?? 0;
        },
        set: async (scope, value) => {
            if (typeof scope !== "string")
                throw new Error("scope must be a string");
            if (typeof value !== "number")
                throw new Error("value must be a number");
            const objectValue = await getObject(key, {});
            objectValue[scope] = value;
            return setObject(key, objectValue);
        },
        increase: (scope, amount) => update(scope, (value) => value + (amount ?? 1)),
        decrease: (scope, amount) => update(scope, (value) => value - (amount ?? 1)),
        all: async () => {
            const objectValue = await getObject(key, {});
            return Object.entries(objectValue).map(([scope, amount]) => ({
                scope,
                amount: amount,
            }));
        },
    };
}

var kv = /*#__PURE__*/Object.freeze({
    __proto__: null,
    createCounter: createCounter,
    createScopedCounter: createScopedCounter,
    getArray: getArray,
    getNumber: getNumber,
    getObject: getObject,
    getText: getText,
    remove: remove,
    setArray: setArray,
    setNumber: setNumber,
    setObject: setObject,
    setText: setText
});

/**
 * Send a chat message to twitch
 *
 * @param message Message to send
 * @returns Promise resolved when the message has sent
 */
function sendChat(message) {
    return Deno.core.ops.op_twitch_send_chat(message);
}
/**
 * Send a twitch chat announcement
 *
 * @param message Message to send
 * @param color Optional message color (Defaults to primary color)
 * @returns Promise resolved when the message has sent
 */
function sendChatAnnouncement(message, color) {
    return Deno.core.ops.op_twitch_send_chat_announcement(message, color ?? "primary");
}
/**
 * Attempts to lookup a twitch user by username
 *
 * @param username Username of the user to get
 * @returns Promise resolved to the twitch user
 */
function getUserByUsername(username) {
    // Validate username before calling API
    if (!isValidUsernameStrict(username)) {
        throw new Error("username is invalid");
    }
    return Deno.core.ops.op_twitch_get_user_by_username(username);
}
/**
 * Triggers a twitch shoutout for the provided use
 *
 * @param userId The ID of the user to shoutout
 * @returns Promise resolved when the shoutout is complete
 */
function shoutout(userId) {
    return Deno.core.ops.op_twitch_send_shoutout(userId);
}
/**
 * Checks if the user is a mod on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a mod
 */
function isModerator(userId) {
    return Deno.core.ops.op_twitch_is_mod(userId);
}
/**
 * Checks if the user is a vip on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a vip
 */
function isVip(userId) {
    return Deno.core.ops.op_twitch_is_vip(userId);
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
async function getFollower(userId) {
    const follower = await Deno.core.ops.op_twitch_get_follower(userId);
    if (follower === null)
        return null;
    return {
        id: follower.user_id,
        name: follower.user_login,
        displayName: follower.user_name,
        followedAt: new Date(follower.followed_at),
    };
}
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
function getUsernameArg(rawArg, validate = false) {
    // Arg not provided
    if (rawArg === undefined || rawArg === null || typeof rawArg !== "string")
        return null;
    let arg = rawArg;
    // Trim whitespace
    arg = arg.trim();
    // Strip @ from mention
    if (arg.startsWith("@"))
        arg = arg.substring(1);
    // Empty
    if (arg.length < 1)
        return null;
    // Apply strict validation
    if (validate && !isValidUsernameStrict(arg))
        return null;
    return arg;
}
/**
 * Applies strict validation against the provided username
 * to ensure that it is a twitch username ensuring the correct
 * character and length requirements
 *
 * @param username The username to check
 * @returns Whether the username is valid
 */
function isValidUsernameStrict(username) {
    if (!username)
        return false;
    const length = username.length;
    // Check length
    if (length < 4 || length > 25)
        return false;
    // Check for leading or trailing underscores
    if (username[0] === "_" || username[length - 1] === "_")
        return false;
    // Iterate through characters to validate
    for (let i = 0; i < length; i++) {
        const char = username[i];
        // Check if character is valid (alphanumeric or underscore)
        const isAlphaNumeric = (char >= "a" && char <= "z") ||
            (char >= "A" && char <= "Z") ||
            (char >= "0" && char <= "9") ||
            char === "_";
        if (!isAlphaNumeric)
            return false;
    }
    return true;
}

var twitch = /*#__PURE__*/Object.freeze({
    __proto__: null,
    getFollower: getFollower,
    getUserByUsername: getUserByUsername,
    getUsernameArg: getUsernameArg,
    isModerator: isModerator,
    isValidUsernameStrict: isValidUsernameStrict,
    isVip: isVip,
    sendChat: sendChat,
    sendChatAnnouncement: sendChatAnnouncement,
    shoutout: shoutout
});

async function request(options) {
    // URL must be defined and a string
    if (options.url === undefined || typeof options.url !== "string") {
        throw new Error("url must be a present and a string");
    }
    let requestBody = undefined;
    const body = options.body;
    if (typeof body === "string") {
        requestBody = { type: "text", value: body };
    }
    else if (typeof body === "object") {
        requestBody = { type: "json", value: body };
    }
    let responseFormat = (options.responseFormat ?? "text").toLowerCase();
    const response = await Deno.core.ops.op_http_request({
        url: options.url,
        method: options.method,
        body: requestBody,
        headers: options.headers,
        timeout: options.timeout,
        response_format: responseFormat,
    });
    return {
        ...response,
        get ok() {
            return Math.floor(response.status / 100) == 2;
        },
    };
}
function get(url, options) {
    return request({ ...options, url, method: "GET" });
}
function post(url, body, options) {
    return request({ ...options, url, method: "POST", body });
}
function put(url, body, options) {
    return request({ ...options, url, method: "PUT", body });
}
function patch(url, body, options) {
    return request({ ...options, url, method: "PATCH", body });
}

var http = /*#__PURE__*/Object.freeze({
    __proto__: null,
    get: get,
    patch: patch,
    post: post,
    put: put,
    request: request
});

/**
 * Requests the list of voices from TTS Monster
 *
 * @returns The list of available voices
 */
function voices() {
    return Deno.core.ops.op_vtftk_tts_get_voices();
}
/**
 * Generate a single voice message using a specific voice
 *
 * @param voice_id The ID of the voice to use
 * @param message The message for the voice to say
 * @returns URL to the voice message file
 */
function generate(voice_id, message) {
    return Deno.core.ops.op_vtftk_tts_generate(voice_id, message);
}
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
function generateParsed(message) {
    return Deno.core.ops.op_vtftk_tts_generate_parsed(message);
}

var tts_monster = /*#__PURE__*/Object.freeze({
    __proto__: null,
    generate: generate,
    generateParsed: generateParsed,
    voices: voices
});

var integrations = /*#__PURE__*/Object.freeze({
    __proto__: null,
    tts_monster: tts_monster
});

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
function playSound(src, volume = 1) {
    return Deno.core.ops.op_vtftk_play_sound(src, volume);
}
/**
 * Plays the provided collection of sound through the overlay
 * one by one, only starts playing the next sound after the
 * first sound completes
 *
 * @param sounds Sequence of sounds to play
 * @returns Promise resolved when the sounds has been sent to the event queue
 */
function playSoundSeq(sounds) {
    return Deno.core.ops.op_vtftk_play_sound_seq(sounds);
}

var vtftk = /*#__PURE__*/Object.freeze({
    __proto__: null,
    playSound: playSound,
    playSoundSeq: playSoundSeq
});

/**
 * This is the core runtime script this is stored as a JS snapshot
 * and all the globals created by this script are exposed when
 * running scripts at runtime
 *
 * This contains helpers, wrapper functions and glue for interacting
 * with the Rust side of the runtime
 */
// API functions provided to the runtime
Object.defineProperty(globalThis, "api", {
    value: {
        twitch,
        kv,
        http,
        logging,
        vtftk,
        integrations,
    },
    writable: false,
    configurable: false,
});
// Copy the logging functions to the commonly known console functions
Object.defineProperty(globalThis, "console", {
    value: {
        log: info,
        info: info,
        error: error,
        debug: debug,
    },
    writable: false,
    configurable: false,
});
Array.prototype.random = function () {
    return this[Math.floor(Math.random() * this.length)];
};
