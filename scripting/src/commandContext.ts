import { runWithContext } from "./context";

export interface CommandContext {
  /**
   * ID of the message
   */
  messageId: string;

  /**
   * Full original message, contains the command prefix
   */
  fullMessage: string;

  /**
   * Message without the command prefix
   */
  message: string;

  /**
   * User who executed the command
   */
  user: TwitchCommandUser;

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

type BaseCommandContext = Omit<
  CommandContext,
  "targetUser" | "targetUserValid"
>;

/**
 * Internal logic, extends the base command context
 * to provide additional getters
 *
 * @param baseContext The base context
 * @returns The extended context
 */
function extendCommandContext(baseContext: BaseCommandContext): CommandContext {
  return {
    ...baseContext,

    // Inject getters for helping with getting the target user
    get targetUser() {
      return api.twitch.getUsernameArg(baseContext.args[0], false);
    },

    get targetUserValid() {
      return api.twitch.getUsernameArg(baseContext.args[0], true);
    },
  };
}

export type TwitchCommandUser = {
  /**
   * ID of the twitch user
   */
  id: string;
  /**
   * Username of the twitch user
   */
  name: string;
  /**
   * Display name of the twitch user
   */
  displayName: string;
};

declare global {
  /**
   * Context for the current command execution, only available within
   * command scripts.
   *
   * With this you can access details about who triggered the command
   * along with the message itself and its arguments.
   */
  const ctx: CommandContext;
}

export function executeCommandOutlet(
  ctx: unknown,
  baseContext: BaseCommandContext,
  userFunction: (ctx: CommandContext) => Promise<unknown>,
): Promise<void> {
  return runWithContext(ctx, async () => {
    const commandCtx = extendCommandContext(baseContext);

    let value;
    try {
      value = await userFunction(commandCtx);
    } catch (err) {
      console.error("error running user command code", err);
      return;
    }

    // Send the chat response if the return value is a string
    if (typeof value === "string") {
      await api.twitch.sendChat(value);
    }
  });
}
