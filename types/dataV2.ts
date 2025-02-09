import type { Uuid } from "./appData";

export type ItemId = Uuid;

export type ItemImageConfig = {
  src: string;
  weight: number;
  scale: number;
  pixelate: boolean;
};

export type ItemWindupConfig = {
  enabled: boolean;
  duration: number;
};

export type Item = {
  id: ItemId;
  name: string;
  config: ItemConfig;
  order: number;
};

export type ItemConfig = {
  image: ItemImageConfig;
  windup: ItemWindupConfig;
};

export type SoundId = Uuid;

export type Sound = {
  id: SoundId;
  name: string;
  src: string;
  volume: number;
  order: number;
};
