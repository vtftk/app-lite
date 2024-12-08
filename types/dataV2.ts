import type {
  CommandOutcome,
  EventOutcome,
  EventTrigger,
  MinimumRequiredRole,
  ThrowableImageConfig,
  Uuid,
} from "./appData";

export type ItemId = Uuid;

export type Item = {
  id: ItemId;
  name: string;
  image: ThrowableImageConfig;
};

export type ItemWithImpactSoundIds = Item & {
  impact_sound_ids: SoundId[];
};

export type ItemWithImpactSounds = Item & { impact_sounds: Sound[] };

export type CreateItem = {
  name: string;
  image: ThrowableImageConfig;
  impact_sounds: SoundId[];
};

export type UpdateItem = {
  itemId: ItemId;
  update: Partial<{
    name: string;
    image: ThrowableImageConfig;
    impact_sounds: SoundId[];
  }>;
};

export type ThrowableConfig = {
  items: ItemWithImpactSoundIds[];
  impact_sounds: Sound[];
};

export type SoundId = Uuid;

export type Sound = {
  id: SoundId;
  name: string;
  src: string;
  volume: number;
};

export type CreateSound = {
  name: string;
  src: string;
  volume: number;
};

export type UpdateSound = {
  soundId: SoundId;
  update: Partial<{
    name: string;
    src: string;
    volume: number;
  }>;
};

export type ScriptId = Uuid;

export type Script = {
  id: ScriptId;
  enabled: boolean;
  name: string;
  script: string;
  events: string[];
};

export type CreateScript = {
  enabled: boolean;
  name: string;
  script: string;
  events: string[];
};

export type UpdateScript = {
  scriptId: ScriptId;
  update: Partial<{
    enabled: boolean;
    name: string;
    script: string;
    events: string[];
  }>;
};

export type CommandId = Uuid;

export type Command = {
  id: Uuid;
  enabled: boolean;
  name: string;
  command: string;
  aliases: string[];
  outcome: CommandOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
};

export type CreateCommand = {
  enabled: boolean;
  name: string;
  command: string;
  aliases: string[];
  outcome: CommandOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
};

export type UpdateCommand = {
  commandId: CommandId;
  update: Partial<{
    enabled: boolean;
    name: string;
    command: string;
    aliases: string[];
    outcome: CommandOutcome;
    cooldown: number;
    require_role: MinimumRequiredRole;
  }>;
};

export type EventId = Uuid;

export type VEvent = {
  id: EventId;
  name: string;
  enabled: boolean;
  trigger: EventTrigger;
  outcome: EventOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
  outcome_delay: number;
};

export type CreateEvent = {
  name: string;
  enabled: boolean;
  trigger: EventTrigger;
  outcome: EventOutcome;
  cooldown: number;
  require_role: MinimumRequiredRole;
  outcome_delay: number;
};

export type UpdateEvent = {
  eventId: EventId;
  update: Partial<{
    name: string;
    enabled: boolean;
    trigger: EventTrigger;
    outcome: EventOutcome;
    cooldown: number;
    require_role: MinimumRequiredRole;
    outcome_delay: number;
  }>;
};
