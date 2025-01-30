import type { Uuid } from "./appData";

export type ItemId = Uuid;

export type ThrowableImageConfig = {
  src: string;
  weight: number;
  scale: number;
  pixelate: boolean;
};

export type Item = {
  id: ItemId;
  name: string;
  image: ThrowableImageConfig;
  order: number;
};

export type SoundId = Uuid;

export type Sound = {
  id: SoundId;
  name: string;
  src: string;
  volume: number;
  order: number;
};
