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

  items: ItemConfig[];
  events: EventConfig[];
  sounds: SoundConfig[];
  scripts: UserScriptConfig[];
  commands: CommandConfig[];
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
  items: ItemConfig[];
  impact_sounds: SoundConfig[];
};

export type ItemConfig = {
  id: Uuid;
  name: string;
  image: ThrowableImageConfig;
  impact_sounds_ids: Uuid[];
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
  | { type: EventTriggerType.Bits; min_bits: number }
  | {
      type: EventTriggerType.Raid;
      min_raiders: number;
    };

export enum ThrowableDataType {
  Throw = "Throw",
  Barrage = "Barrage",
}

export type ThrowableDataThrow = {
  throwable_ids: Uuid[];
  amount: number;
};

export type ThrowableDataBarrage = {
  throwable_ids: Uuid[];
  amount_per_throw: number;
  frequency: number;
  amount: number;
};

export type ThrowableData =
  | ({ type: ThrowableDataType.Throw } & ThrowableDataThrow)
  | ({ type: ThrowableDataType.Barrage } & ThrowableDataBarrage);

export enum BitsAmountType {
  Fixed = "Fixed",
  Dynamic = "Dynamic",
}

export type BitsAmountFixed = {
  amount: number;
};

export type BitsAmountDynamic = {
  max_amount: number;
};

export type BitsAmount =
  | ({ type: BitsAmountType.Fixed } & BitsAmountFixed)
  | ({ type: BitsAmountType.Dynamic } & BitsAmountDynamic);

export enum EventOutcomeType {
  ThrowBits = "ThrowBits",
  Throwable = "Throwable",
  TriggerHotkey = "TriggerHotkey",
  PlaySound = "PlaySound",
}

export type EventOutcomeBits = {
  _1: Option<Uuid>;
  _100: Option<Uuid>;
  _1000: Option<Uuid>;
  _5000: Option<Uuid>;
  _10000: Option<Uuid>;
  amount: BitsAmount;
};
export type EventOutcomeThrowable = { data: ThrowableData };
export type EventOutcomeTriggerHotkey = { hotkey_id: Uuid };
export type EventOutcomePlaySound = { sound_id: Uuid };

export type EventOutcome =
  | ({ type: EventOutcomeType.ThrowBits } & EventOutcomeBits)
  | ({ type: EventOutcomeType.Throwable } & EventOutcomeThrowable)
  | ({ type: EventOutcomeType.TriggerHotkey } & EventOutcomeTriggerHotkey)
  | ({ type: EventOutcomeType.PlaySound } & EventOutcomePlaySound);

export type EventOutcomeVariant<T extends EventOutcomeType> = Extract<
  EventOutcome,
  { type: T }
>;

export type CustomReward = any;

export type UserScriptConfig = {
  id: Uuid;
  enabled: boolean;
  name: string;
  script: string;
  events: string[];
};

export type CommandConfig = {
  id: Uuid;
  enabled: boolean;
  name: string;
  command: string;
  aliases: string[];
  outcome: CommandOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
};

export enum CommandOutcomeType {
  Template = "Template",
  Script = "Script",
}

export type CommandOutcomeTemplate = { message: string };
export type CommandOutcomeScript = { script: string };

export type CommandOutcome =
  | ({ type: CommandOutcomeType.Template } & CommandOutcomeTemplate)
  | ({ type: CommandOutcomeType.Script } & CommandOutcomeScript);
