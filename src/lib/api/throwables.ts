import type { AppData, ItemConfig, ThrowableConfig } from "$shared/appData";
import { invoke } from "@tauri-apps/api/core";

/**
 * Collect all of the impact sounds for the provided
 * list of item configs
 *
 * @param appData The app data to pick sounds from
 * @param items The items to pick sounds for
 * @returns The collection of sounds
 */
export function getItemsImpactSounds(appData: AppData, items: ItemConfig[]) {
  return appData.sounds.filter((sound) =>
    items.some((item) => item.impact_sounds_ids.includes(sound.id))
  );
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
export function testThrow(
  appData: AppData,
  itemIds: string[],
  amount: number = 1
) {
  const items = appData.items.filter((item) => itemIds.includes(item.id));
  const impact_sounds = getItemsImpactSounds(appData, items);

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
export function testThrowBarrage(
  appData: AppData,
  itemIds: string[],
  amount: number = 50,
  amountPerThrow: number = 2,
  frequency: number = 100
) {
  const items = appData.items.filter((item) => itemIds.includes(item.id));
  const impact_sounds = getItemsImpactSounds(appData, items);

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
