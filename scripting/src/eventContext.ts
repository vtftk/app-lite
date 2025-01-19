import { runWithContext } from "./context";

export interface TwitchEventUser {
  id: string;
  name: string;
  displayName: string;
}

export type SubscriptionTier = "1000" | "2000" | "3000" | "Prime" | string;

export type EventData = {
  /**
   * User the event was triggered by.
   *
   * Some events do not have a user those events will have null instead
   */
  user: TwitchEventUser | null;
};

interface RedeemInputData {
  /**
   * Unique ID for the specific redemption event
   */
  redemptionId: string;
  /**
   * Name of the redeem
   */
  rewardName: string;
  /**
   * Unique ID for the redeem
   */
  rewardId: string;
  /**
   * Channel points cost for the redeem
   */
  cost: number;
  /**
   * User provided message if the redeem
   * requests a message
   */
  userInput: string;
}

interface BitsInputData {
  /**
   * Number of bits gifted
   */
  bits: number;
  /**
   * Whether the bits were gifted anonymously
   */
  anonymous: boolean;
  /**
   * Message provided with the gifted bits
   */
  message: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface FollowInputData {}

interface SubscriptionInputData {
  /**
   * Tier of the subscription
   */
  tier: SubscriptionTier;
  /**
   * Whether the subscription was gifted
   */
  isGift: boolean;
}

interface GiftedSubscriptionInputData {
  /**
   * Tier of subscription gifted
   */
  tier: SubscriptionTier;
  /**
   * The number of subscriptions gifted by this user in the channel.
   * Null when anonymous gifter
   */
  cumulativeTotal: number | null;
  /**
   * Whether it was gifted anonymously
   */
  anonymous: boolean;
  /**
   * Total number of subs gifted
   */
  total: number;
}

interface ResubscriptionInputData {
  /**
   * The total number of months the user has been subscribed to the channel.
   */
  cumulativeMonths: number;
  /**
   * The month duration of the subscription. (The gifted amount)
   */
  durationMonths: number;
  /**
   * User message attached to the resubscription
   */
  message: string;
  /**
   * The number of consecutive months the user’s current subscription has been active.
   * This value is null if the user has opted out of sharing this information.
   */
  streakMonths: number | null;
  /**
   * Tier resubscribed at
   */
  tier: SubscriptionTier;
}

interface ChatInputData {
  /**
   * ID of the chat message
   */
  messageId: string;
  /**
   * The chat message content
   */
  message: string;
  /**
   * Individual chat fragments
   */
  fragments: ChatFragment[];
  /**
   * Cheered bits amount if the message contained a bits cheer
   */
  cheer: number | null;
}

interface Cheermote {
  /**
   * The name portion of the Cheermote string that you use in chat to cheer Bits.
   *
   * The full Cheermote string is the concatenation of {prefix} + {number of Bits}.
   * For example, if the prefix is “Cheer” and you want to cheer 100 Bits, the full Cheermote string is Cheer100.
   * When the Cheermote string is entered in chat, Twitch converts it to the image associated with the Bits tier that was cheered.
   */
  prefix: string;
  /**
   * The amount of bits cheered.
   */
  bits: number;
  /**
   * The tier level of the cheermote.
   */
  tier: number;
}

interface Emote {
  /**
   * An ID that uniquely identifies this emote.
   */
  id: string;
  /**
   * An ID that identifies the emote set that the emote belongs to.
   */
  emote_set_id: string;
  /**
   * The ID of the broadcaster who owns the emote.
   */
  owner_id: string;
  /**
   * The formats that the emote is available in. For example, if the emote is available only as a static PNG, the array contains only static. But if the emote is available as a static PNG and an animated GIF, the array contains static and animated. The possible formats are:
   * * `animated` — An animated GIF is available for this emote.
   * * `static` — A static PNG file is available for this emote.
   */
  format: ("animated" | "static")[];
}

interface Mention {
  /**
   * The user ID of the mentioned user. (Unique ID)
   */
  user_id: string;
  /**
   * The user name of the mentioned user. (Display name)
   */
  user_name: string;
  /**
   * The user login of the mentioned user. (Username)
   */
  user_login: string;
}

type ChatFragment =
  | {
      type: "Cheermote";
      text: string;
      cheermote: Cheermote;
    }
  | { type: "Emote"; text: string; emote: Emote }
  | { type: "Mention"; text: string; mention: Mention }
  | { type: "Text"; text: string };

interface RaidInputData {
  /**
   * Number of viewers present in the raid
   */
  viewers: number;
}

interface AdBreakBeginInputData {
  /**
   * Duration in seconds for the ad break that is starting
   */
  durationSeconds: number;
}

interface ShoutoutReceiveInputData {
  /**
   * Number of viewers that have seen the shoutout
   */
  viewerCount: number;
}

type EventInputData =
  | RedeemInputData
  | BitsInputData
  | FollowInputData
  | SubscriptionInputData
  | GiftedSubscriptionInputData
  | ResubscriptionInputData
  | ChatInputData
  | RaidInputData
  | AdBreakBeginInputData
  | ShoutoutReceiveInputData;

export type EventContext = EventData & EventInputData;

declare global {
  /**
   * Event data, only available within the context of a event outcome script
   */
  const event: EventContext;
}

export function executeEventOutlet(
  ctx: unknown,
  eventContext: EventContext,
  userFunction: (event: EventContext) => Promise<unknown>,
): Promise<void> {
  return runWithContext(ctx, async () => {
    try {
      await userFunction(eventContext);
    } catch (err) {
      console.error("error running user event code", err);
    }
  });
}
