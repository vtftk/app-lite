type Option<T> = T | null;

type Uuid = string;

export type MinMax = {
  min: number;
  max: number;
};

export type AppData = {
  throwables_config: ThrowablesConfig;
  items_config: ItemsConfig;
  model_config: ModelConfig;
  vtube_studio_config: VTubeStudioConfig;
  models: Record<ModelId, ModelData>;

  items: ThrowableConfig[];
  events: EventConfig[];
  sounds: SoundConfig[];
};

export type ThrowablesConfig = {
  duration: number;
  spin_speed: MinMax;
  throw_angle: MinMax;
  direction: ThrowDirection;
  impact_delay: number;
};

export enum ThrowDirection {
  Random = "Random",
  LeftOnly = "LeftOnly",
  RightOnly = "RightOnly",
}

export type ModelId = string;

export type ModelData = {
  x: MinMax;
  y: MinMax;
};

export type ItemsConfig = {
  global_volume: number;
  item_scale: MinMax;
};

export type ModelConfig = {
  model_return_time: number;
  eyes_on_hit: EyesMode;
};

export enum EyesMode {
  Unchanged = "Unchanged",
  Opened = "Opened",
  Closed = "Closed",
}

export type VTubeStudioConfig = {
  host: string;
  port: number;
};

export type ThrowableConfig = {
  id: Uuid;
  name: string;
  image: ThrowableImageConfig;
  sound: Option<ImpactSoundConfig>;
};

export type ThrowableImageConfig = {
  src: string;
  weight: number;
  scale: number;
  pixelate: boolean;
};

export type ImpactSoundConfig = {
  src: string;
  volume: number;
};

export type SoundConfig = {
  id: Uuid;
  name: string;
  src: string;
  volume: number;
};

export type EventConfig = {
  id: Uuid;
  name: string;
  enabled: boolean;
  trigger: EventTrigger;
  outcome: EventOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
  outcome_delay: number;
};

export enum MinimumRequiredRole {
  None = "None",
  Vip = "Vip",
  Mod = "Mod",
}

export const MINIMUM_REQUIRED_ROLE_VALUES = [
  MinimumRequiredRole.None,
  MinimumRequiredRole.Vip,
  MinimumRequiredRole.Mod,
] as const;

export const MINIMUM_REQUIRED_ROLE_NAMES: Record<MinimumRequiredRole, string> =
  {
    [MinimumRequiredRole.None]: "None",
    [MinimumRequiredRole.Vip]: "Vip",
    [MinimumRequiredRole.Mod]: "Moderator",
  } as const;

export enum EventTriggerType {
  Redeem = "Redeem",
  Command = "Command",
  Follow = "Follow",
  Subscription = "Subscription",
  GiftedSubscription = "GiftedSubscription",
  Bits = "Bits",
  Raid = "Raid",
}

export const EVENT_TRIGGER_TYPES = [
  EventTriggerType.Redeem,
  EventTriggerType.Command,
  EventTriggerType.Follow,
  EventTriggerType.Subscription,
  EventTriggerType.GiftedSubscription,
  EventTriggerType.Bits,
  EventTriggerType.Raid,
] as const;

export const EVENT_TRIGGER_NAMES: Record<EventTriggerType, string> = {
  [EventTriggerType.Redeem]: "Redeem",
  [EventTriggerType.Command]: "Command",
  [EventTriggerType.Follow]: "Follow",
  [EventTriggerType.Subscription]: "Subscription",
  [EventTriggerType.GiftedSubscription]: "Gifted Subscription",
  [EventTriggerType.Bits]: "Bits",
  [EventTriggerType.Raid]: "Raid",
} as const;

export type EventTrigger =
  | {
      type: EventTriggerType.Redeem;
      reward_id: string;
    }
  | { type: EventTriggerType.Command; message: string }
  | { type: EventTriggerType.Follow }
  | { type: EventTriggerType.Subscription }
  | { type: EventTriggerType.GiftedSubscription }
  | { type: EventTriggerType.Bits; min_bits: number; max_throws: number }
  | {
      type: EventTriggerType.Raid;
      min_raiders: number;
      throws: MinMax;
    };

export enum ThrowableSelectionType {
  Random = "Random",
  Selection = "Selection",
}

export type ThrowableSelection =
  | { type: ThrowableSelectionType.Random }
  | { type: ThrowableSelectionType.Selection; throwable_ids: Uuid[] };

export const THROWABLE_SELECTION_TYPES = [
  ThrowableSelectionType.Random,
  ThrowableSelectionType.Selection,
] as const;

export const THROWABLE_SELECTION_NAMES: Record<ThrowableSelectionType, string> =
  {
    [ThrowableSelectionType.Random]: "Random",
    [ThrowableSelectionType.Selection]: "Choose specific throwables",
  } as const;

export enum EventOutcomeType {
  Throwable = "Throwable",
  ThrowableBarrage = "ThrowableBarrage",
  TriggerHotkey = "TriggerHotkey",
  PlaySound = "PlaySound",
}

export type EventOutcome =
  | {
      type: EventOutcomeType.Throwable;
      selection: ThrowableSelection;
      amount: number;
    }
  | {
      type: EventOutcomeType.ThrowableBarrage;
      selection: ThrowableSelection;
      frequency: number;
      amount: number;
    }
  | { type: EventOutcomeType.TriggerHotkey; hotkey_id: Uuid }
  | { type: EventOutcomeType.PlaySound; sound_id: Uuid };

export type EventOutcomeVariant<T extends EventOutcomeType> = Extract<
  EventOutcome,
  { type: T }
>;

export const EVENT_OUTCOME_TYPES = [
  EventOutcomeType.Throwable,
  EventOutcomeType.ThrowableBarrage,
  EventOutcomeType.TriggerHotkey,
  EventOutcomeType.PlaySound,
] as const;

export const EVENT_OUTCOME_NAMES: Record<EventOutcomeType, string> = {
  [EventOutcomeType.Throwable]: "Throwable",
  [EventOutcomeType.ThrowableBarrage]: "Throwable Barrage",
  [EventOutcomeType.TriggerHotkey]: "Trigger Hotkey",
  [EventOutcomeType.PlaySound]: "Play Sound",
} as const;

export type CustomReward = any;
