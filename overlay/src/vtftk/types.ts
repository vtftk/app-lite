import { Item, Sound, SoundId } from "$shared/dataV2";

export * from "$shared/dataV2";
export * from "$shared/appData";
export * from "$shared/runtimeAppData";

export const enum ThrowItemConfigType {
  Barrage = "Barrage",
  All = "All",
}

export type ThrowItemConfig =
  | { type: ThrowItemConfigType.All; amount: number }
  | {
      type: ThrowItemConfigType.Barrage;
      amount_per_throw: number;
      amount: number;
      frequency: number;
    };

export type ItemWithSoundIds = Item & {
  impact_sound_ids: SoundId[];
};

export type ItemWithSounds = {
  items: ItemWithSoundIds[];
  sounds: Sound[];
};
