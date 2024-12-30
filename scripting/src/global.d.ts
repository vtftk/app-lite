/// <reference no-default-lib="true" />

import * as logging from "./logging";
import * as kv from "./kv";
import * as twitch from "./twitch";
import * as http from "./http";
import * as integrations from "./integrations";
import * as vtftk from "./vtftk";

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

export {};

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
