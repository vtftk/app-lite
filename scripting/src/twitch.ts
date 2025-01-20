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

/**
 * Send a chat message to twitch
 *
 * @param message Message to send
 * @returns Promise resolved when the message has sent
 */
export function sendChat(message: string): Promise<void> {
  return Deno.core.ops.op_twitch_send_chat(message);
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
export function sendChatAnnouncement(
  message: string,
  color: TwitchAnnouncementColor = "primary",
): Promise<void> {
  return Deno.core.ops.op_twitch_send_chat_announcement(message, color);
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

/**
 * Attempts to lookup a twitch user by username
 *
 * @param username Username of the user to get
 * @returns Promise resolved to the twitch user
 */
export function getUserByUsername(
  username: TwitchUsername,
): Promise<TwitchUser | null> {
  // Validate username before calling API
  if (!isValidUsernameStrict(username)) {
    throw new Error("username is invalid");
  }

  return Deno.core.ops.op_twitch_get_user_by_username(username);
}

/**
 * Triggers a twitch shoutout for the provided user
 *
 * @param userId The ID of the user to shoutout
 * @returns Promise resolved when the shoutout is complete
 */
export function shoutout(userId: TwitchUserId): Promise<void> {
  assertUserId(userId);

  return Deno.core.ops.op_twitch_send_shoutout(userId);
}

/**
 * Checks if the user is a mod on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a mod
 */
export function isModerator(userId: TwitchUserId): Promise<boolean> {
  assertUserId(userId);

  return Deno.core.ops.op_twitch_is_mod(userId);
}

/**
 * Checks if the user is a vip on the twitch channel
 *
 * @param userId The ID of the user
 * @returns Promise resolved with whether the user is a vip
 */
export function isVip(userId: TwitchUserId): Promise<boolean> {
  assertUserId(userId);

  return Deno.core.ops.op_twitch_is_vip(userId);
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

  // Internal format for a twitch follower
  interface _TwitchFollower {
    followed_at: string;
    user_id: string;
    user_login: string;
    user_name: string;
  }

  const follower: _TwitchFollower | null =
    await Deno.core.ops.op_twitch_get_follower(userId);

  if (follower === null) return null;

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
export function deleteChatMessage(messageId: string): Promise<void> {
  if (messageId === undefined) throw new Error("messageId must be provided");
  if (typeof messageId !== "string") throw new Error("messageId is invalid");

  return Deno.core.ops.op_twitch_delete_chat_message(messageId);
}

/**
 * Deletes all messages from chat
 *
 * @returns Promise resolved when the message
 */
export function deleteAllChatMessages(): Promise<void> {
  return Deno.core.ops.op_twitch_delete_all_chat_messages();
}

/**
 * Creates a new stream marker
 *
 * @param description Optional description for the stream marker
 * @returns Promise resolved when the marker is created
 */
export function createStreamMarker(description?: string): Promise<void> {
  return Deno.core.ops.op_twitch_create_stream_marker(description);
}
