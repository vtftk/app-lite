import { invoke } from "@tauri-apps/api/core";

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
  return invoke<void>("test_throw", {
    itemIds,
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
  frequency: number = 100,
) {
  return invoke<void>("test_throw_barrage", {
    itemIds,
    amount,
    amountPerThrow,
    frequency,
  });
}
