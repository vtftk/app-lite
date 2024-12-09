import { invoke } from "@tauri-apps/api/core";
import { getItemById } from "./items";
import type {
  ItemWithImpactSoundIds,
  Sound,
  ThrowableConfig,
} from "$shared/dataV2";

async function getItemsWithSounds(
  itemIds: string[]
): Promise<{ items: ItemWithImpactSoundIds[]; impact_sounds: Sound[] }> {
  const rawItems = await Promise.all(
    itemIds.map((itemId) => getItemById(itemId))
  );

  const impact_sounds: Sound[] = [];
  const items: ItemWithImpactSoundIds[] = [];

  for (const rawItem of rawItems) {
    if (rawItem === null) continue;

    const { impact_sounds: item_impact_sounds, ...item } = rawItem;
    items.push({
      ...item,
      impact_sound_ids: impact_sounds.map((sound) => sound.id),
    });

    // Add non duplicate sounds
    for (const impact_sound of item_impact_sounds) {
      const existing = impact_sounds.find(
        (sound) => sound.id === impact_sound.id
      );
      if (existing !== undefined) continue;
      impact_sounds.push(impact_sound);
    }
  }

  return {
    items,
    impact_sounds,
  };
}

/**
 * Throws a test item (or items depending on amount) from the selection of
 * items with an ID present in itemIds
 *
 * @param appData The app data to pick the selection from
 * @param itemIds The item ID's to throw
 * @param amount The amount of items to throw
 * @returns Promise that completes when the throw has been sent
 */
export async function testThrow(itemIds: string[], amount: number = 1) {
  const { items, impact_sounds } = await getItemsWithSounds(itemIds);
  const throwable: ThrowableConfig = {
    items,
    impact_sounds,
  };

  return invoke<void>("test_throw", {
    config: throwable,
    amount,
  });
}

/**
 * Throws a test barrage of items from the selection of
 * items with an ID present in itemIds
 *
 * @param appData The app data to pick the selection from
 * @param itemIds The item ID's to throw
 * @param amount The total amount of items to throw
 * @param amountPerThrow Amount to throw per barrage
 * @param frequency Time between each barrage
 * @returns Promise that completes when the throw has been sent
 */
export async function testThrowBarrage(
  itemIds: string[],
  amount: number = 50,
  amountPerThrow: number = 2,
  frequency: number = 100
) {
  const { items, impact_sounds } = await getItemsWithSounds(itemIds);
  const throwable: ThrowableConfig = {
    items,
    impact_sounds,
  };

  return invoke<void>("test_throw_barrage", {
    config: throwable,
    amount,
    amountPerThrow,
    frequency,
  });
}
