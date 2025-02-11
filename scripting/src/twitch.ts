import { HttpResponse } from "./http";

/**
 * Helper to assert the validity of a user ID before
 * sending it to actual APIs
 *
 * @param userId The user ID to check
 */
function assertUserId(userId: TwitchUserId) {
  if (userId === undefined) throw new Error("userId must be provided");
  if (typeof userId !== "string") throw new Error("userId is invalid");
}

type TwitchCredentials = {
  // Access token
  token: string;

  // User ID
  user_id: string;

  // Client ID being used
  client_id: string;
};

/**
 * @internal
 *
 * Requests currently authenticated users twitch access
 * token
 */
async function getCredentials(): Promise<TwitchCredentials> {
  const credentials: TwitchCredentials | null =
    await Deno.core.ops.op_twitch_get_credentials();
  if (credentials === null) throw new TwitchNotAuthenticated();
  return credentials;
}

export class TwitchNotAuthenticated extends Error {
  constructor() {
    super("not authenticated");
  }
}

function getTwitchHeaders(credentials: TwitchCredentials) {
  return {
    authorization: `Bearer ${credentials.token}`,
    "client-id": credentials.client_id,
  };
}

/**
 * Send a chat message to twitch
 *
 * @param message Message to send
 * @returns Promise resolved when the message has sent
 */
export async function sendChat(message: string): Promise<void> {
  const credentials = await getCredentials();

  const response = await api.http.post(
    "https://api.twitch.tv/helix/chat/messages",
    {
      broadcaster_id: credentials.user_id,
      sender_id: credentials.user_id,
      message,
    },
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);
}

/**
 * Color for the twitch announcement banner
 */
export type TwitchAnnouncementColor =
  | "blue"
  | "green"
  | "orange"
  | "purple"
  | "primary";

/**
 * Send a twitch chat announcement
 *
 * @param message Message to send
 * @param color Optional message color (Defaults to primary color)
 * @returns Promise resolved when the message has sent
 */
export async function sendChatAnnouncement(
  message: string,
  color: TwitchAnnouncementColor = "primary",
): Promise<void> {
  const credentials = await getCredentials();
  const userId = credentials.user_id;

  const response = await api.http.post(
    `https://api.twitch.tv/helix/chat/announcements?broadcaster_id=${userId}&moderator_id=${userId}`,
    {
      message,
      color,
    },
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);
}

export type TwitchUserId = string;

export type TwitchUsername = string;

export interface TwitchUser {
  /**
   * ID of the user
   */
  id: TwitchUserId;
  /**
   * Twitch username
   */
  name: string;
  /**
   * Twitch display name
   */
  displayName: string;
  /**
   * URL for the twitch user profile image
   */
  profileImageUrl: string;
}

function mapTwitchUser(raw: RawTwitchUser): TwitchUser {
  return {
    id: raw.id,
    name: raw.login,
    displayName: raw.display_name,
    profileImageUrl: raw.profile_image_url,
  };
}

interface RawTwitchUser {
  id: string;
  login: string;
  display_name: string;
  profile_image_url: string;
}

async function _getTwitchUsersByIds(ids: string[]): Promise<TwitchUser[]> {
  const credentials = await getCredentials();

  const query = ids.map((id) => `id=${id}`).join("&");

  const response = await api.http.get(
    `https://api.twitch.tv/helix/users?${query}`,
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);

  const body = response.body as { data: RawTwitchUser[] };
  return body.data.map(mapTwitchUser);
}

async function getTwitchUsersByUsernames(
  logins: string[],
): Promise<TwitchUser[]> {
  const credentials = await getCredentials();

  const query = logins.map((name) => `login=${name}`).join("&");

  const response = await api.http.get(
    `https://api.twitch.tv/helix/users?${query}`,
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);

  const body = response.body as { data: RawTwitchUser[] };
  return body.data.map(mapTwitchUser);
}

/**
 * Attempts to lookup a twitch user by username
 *
 * @param username Username of the user to get
 * @returns Promise resolved to the twitch user
 */
export async function getUserByUsername(
  username: TwitchUsername,
): Promise<TwitchUser | null> {
  // Validate username before calling API
  if (!isValidUsernameStrict(username)) {
    throw new Error("username is invalid");
  }

  const users = await getTwitchUsersByUsernames([username]);
  if (users.length < 1) return null;
  return users[0];
}

export class TwitchError extends Error {
  constructor(response: HttpResponse<unknown>) {
    super(
      `Twitch Error (${response.status}): ${api.logging.stringify(response.body)}`,
    );
  }
}

/**
 * Triggers a twitch shoutout for the provided user
 *
 * @param userId The ID of the user to shoutout
 * @returns Promise resolved when the shoutout is complete
 */
export async function shoutout(userId: TwitchUserId): Promise<void> {
  assertUserId(userId);

  const credentials = await getCredentials();
  const broadcasterId = credentials.user_id;

  const response = await api.http.post(
    `https://api.twitch.tv/helix/chat/shoutouts?from_broadcaster_id=${broadcasterId}&to_broadcaster_id=${userId}&moderator_id=${broadcasterId}`,
    {},
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);
}

/**
 * Checks if the user is a mod on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a mod
 */
export async function isModerator(userId: TwitchUserId): Promise<boolean> {
  assertUserId(userId);

  const credentials = await getCredentials();
  const broadcasterId = credentials.user_id;

  const response = await api.http.get(
    `https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=${broadcasterId}&user_id=${userId}`,
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);

  const body = response.body as { data: unknown[] };
  return body.data.length > 0;
}

/**
 * Checks if the user is a vip on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a vip
 */
export async function isVip(userId: TwitchUserId): Promise<boolean> {
  assertUserId(userId);

  const credentials = await getCredentials();
  const broadcasterId = credentials.user_id;

  const response = await api.http.get(
    `https://api.twitch.tv/helix/channels/vips?broadcaster_id=${broadcasterId}&user_id=${userId}`,
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);

  const body = response.body as { data: unknown[] };
  return body.data.length > 0;
}

/**
 * Checks if the user is a follower on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a follower
 */
export async function isFollower(userId: TwitchUserId): Promise<boolean> {
  const follower = await getFollower(userId);
  return follower !== null;
}

export interface TwitchFollower {
  /**
   * ID of the user
   */
  id: TwitchUserId;

  /**
   *  User twitch username
   */
  name: TwitchUsername;

  /**
   * User display name
   */
  displayName: string;

  /**
   * Date time when the user followed the broadcaster
   */
  followedAt: Date;
}

// Internal format for a twitch follower
interface RawTwitchFollower {
  followed_at: string;
  user_id: string;
  user_login: string;
  user_name: string;
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
export async function getFollower(
  userId: TwitchUserId,
): Promise<TwitchFollower | null> {
  assertUserId(userId);

  const credentials = await getCredentials();
  const broadcasterId = credentials.user_id;

  const response = await api.http.get(
    `https://api.twitch.tv/helix/channels/followers?broadcaster_id=${broadcasterId}&user_id=${userId}`,
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);

  const body = response.body as { data: RawTwitchFollower[] };

  const follower: RawTwitchFollower | undefined = body.data[0];

  if (follower === undefined) return null;

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
export function getUsernameArg(
  rawArg: unknown,
  validate = false,
): string | null {
  // Arg not provided
  if (rawArg === undefined || rawArg === null || typeof rawArg !== "string")
    return null;

  let arg = rawArg as string;

  // Trim whitespace
  arg = arg.trim();

  // Strip @ from mention
  if (arg.startsWith("@")) arg = arg.substring(1);

  // Empty
  if (arg.length < 1) return null;

  // Apply strict validation
  if (validate && !isValidUsernameStrict(arg)) return null;

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
export function isValidUsernameStrict(username: TwitchUsername): boolean {
  if (!username) return false;

  const length = username.length;

  // Check length
  if (length < 4 || length > 25) return false;

  // Check for leading or trailing underscores
  if (username[0] === "_" || username[length - 1] === "_") return false;

  // Iterate through characters to validate
  for (let i = 0; i < length; i++) {
    const char = username[i];

    // Check if character is valid (alphanumeric or underscore)
    const isAlphaNumeric =
      (char >= "a" && char <= "z") ||
      (char >= "A" && char <= "Z") ||
      (char >= "0" && char <= "9") ||
      char === "_";

    if (!isAlphaNumeric) return false;
  }

  return true;
}

/**
 * Delete a specific message from chat
 *
 * @param messageId ID of the chat message to delete
 * @returns Promise resolved when the message is deleted
 */
export async function deleteChatMessage(messageId: string): Promise<void> {
  if (messageId === undefined) throw new Error("messageId must be provided");
  if (typeof messageId !== "string") throw new Error("messageId is invalid");

  const credentials = await getCredentials();
  const userId = credentials.user_id;

  const response = await api.http.request({
    url: `https://api.twitch.tv/helix/moderation/chat?broadcaster_id=${userId}&moderator_id=${userId}&message_id=${messageId}`,
    responseFormat: "json",
    headers: getTwitchHeaders(credentials),
  });

  if (!response.ok) throw new TwitchError(response);
}

/**
 * Deletes all messages from chat
 *
 * @returns Promise resolved when the message
 */
export async function deleteAllChatMessages(): Promise<void> {
  const credentials = await getCredentials();
  const userId = credentials.user_id;

  const response = await api.http.request({
    url: `https://api.twitch.tv/helix/moderation/chat?broadcaster_id=${userId}&moderator_id=${userId}`,
    responseFormat: "json",
    headers: getTwitchHeaders(credentials),
  });

  if (!response.ok) throw new TwitchError(response);
}

/**
 * Creates a new stream marker
 *
 * @param description Optional description for the stream marker
 * @returns Promise resolved when the marker is created
 */
export async function createStreamMarker(description?: string): Promise<void> {
  const credentials = await getCredentials();
  const response = await api.http.post(
    `https://api.twitch.tv/helix/streams/markers`,
    {
      user_id: credentials.user_id,
      description,
    },
    {
      responseFormat: "json",
      headers: getTwitchHeaders(credentials),
    },
  );

  if (!response.ok) throw new TwitchError(response);
}
