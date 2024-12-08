import type { ThrowableImageConfig, Uuid } from "./appData";

export type ItemId = Uuid;

export type Item = {
  id: ItemId;
  name: string;
  image: ThrowableImageConfig;
};

export type ItemWithImpactSounds = Item & { impact_sounds: Sound[] };

export type SoundId = Uuid;

export type Sound = {
  id: SoundId;
  name: string;
  src: string;
  volume: number;
};

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
  items: Item[];
  impact_sounds: Sound[];
};
