export interface CommandContext {
  // ID of the message
  messageId: string;

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

declare global {
  /**
   * Context for the current command execution, only available within
   * command scripts
   */
  const ctx: CommandContext;
}
