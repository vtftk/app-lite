import { runWithContext } from "./context";

export interface TwitchEventUser {
  id: string;
  name: string;
  display_name: string;
}

export type SubscriptionTier = "1000" | "2000" | "3000" | "Prime" | string;

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
  follow: object;
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
    message_id: string;
    message: string;
    fragments: unknown[];
    cheer: number | null;
  };
  raid: {
    viewers: number;
  };
};

export type EventInputValue = EventInputData[keyof EventInputData];

export type EventContext = EventData & EventInputValue;

declare global {
  /**
   * Event data, only available within the context of a event outcome script
   */
  const event: EventContext;
}

export function createEventOutlet(
  userFunction: (event: EventContext) => Promise<unknown>,
) {
  return (ctx: unknown, eventContext: EventContext): Promise<unknown> => {
    return runWithContext(ctx, userFunction, eventContext);
  };
}
