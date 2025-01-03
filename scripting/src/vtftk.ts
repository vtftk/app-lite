/**
 * Sound stored within VTFTK
 *
 * @member id Unique ID of the sound
 * @member name Name of the sound
 * @member src URL to the sound src
 * @member volume Base volume for the sound
 * @member order Order the sound appears in the UI
 * @member created_at When the sound was created
 */
export interface SoundModel {
  id: string;
  name: string;
  src: string;
  volume: number;
  order: number;
  created_at: string;
}

/**
 * Plays the provided sound through the overlay
 *
 * If you are playing multiple sounds that need to be triggered
 * one after the other use {@see playSoundSeq} instead, play sound
 * promise completes after its been queued not after its finished
 * playing
 *
 * @param src The src URL for the sound file
 * @param volume The volume to play the sound at
 * @returns Promise resolved when the sound has been sent to the event queue
 */
export function playSound(src: string, volume: number = 1): Promise<void> {
  return Deno.core.ops.op_vtftk_play_sound(src, volume);
}

interface SoundSeq {
  // The src URL for the sound file
  src: string;
  // The volume to play the sound at
  volume: number;
}

/**
 * Plays the provided collection of sound through the overlay
 * one by one, only starts playing the next sound after the
 * first sound completes
 *
 * @param sounds Sequence of sounds to play
 * @returns Promise resolved when the sounds has been sent to the event queue
 */
export function playSoundSeq(sounds: SoundSeq[]): Promise<void> {
  return Deno.core.ops.op_vtftk_play_sound_seq(sounds);
}

/**
 * Play a sound by name
 *
 * @param name The name of the sound to play
 * @param ignoreCase Whether to ignore case when matching the sound
 * @param volume Optionally use a different sound volume
 * @returns Promise resolved when the sounds has been queued
 */
export async function playSoundByName(
  name: string,
  ignoreCase: boolean = false,
  volume?: number,
): Promise<void> {
  const sound = await getSoundByName(name, ignoreCase);
  if (sound === null) return;

  return playSound(sound.src, volume ?? sound.volume);
}

/**
 * Play a sound by ID
 *
 * @param id ID of the sound to play
 * @param volume Optionally use a different sound volume
 * @returns Promise resolved when the sounds has been queued
 */
export async function playSoundByID(
  id: string,
  volume?: number,
): Promise<void> {
  const sound = await getSoundByID(id);
  if (sound === null) return;

  return playSound(sound.src, volume ?? sound.volume);
}

/**
 * Same as {@see getSoundByName} but returns either the first
 * found sound or none if none were found
 *
 * @param name The name to search for
 * @param ignoreCase Whether to ignore the name casing
 * @returns The sound that was found or null if none were found
 */
export async function getSoundByName(
  name: string,
  ignoreCase: boolean = false,
): Promise<SoundModel | null> {
  const sounds = await getSoundsByName(name, ignoreCase);
  if (sounds.length < 1) return null;
  return sounds[0];
}

/**
 * Finds all sounds stored in VTFTK with a name that
 * matches the provided name
 *
 * @param name The name to search for
 * @param ignoreCase Whether to ignore the name casing
 * @returns Promise resolved to the list of sounds found
 */
export function getSoundsByName(
  name: string,
  ignoreCase: boolean = false,
): Promise<SoundModel[]> {
  return getSoundsByNames([name], ignoreCase);
}

/**
 * Collects a list of sounds by names from a list of names
 *
 * @param names The names to find
 * @param ignoreCase Whether to ignore name casing when searching
 * @returns Promise resolved to the list of sounds found
 */
export function getSoundsByNames(
  names: string[],
  ignoreCase: boolean = false,
): Promise<SoundModel[]> {
  return Deno.core.ops.op_vtftk_get_sounds_by_names(names, ignoreCase);
}

/**
 * Find a sound using its unique ID
 *
 * @param id The ID of the sound
 * @returns The sound that was found or null if none were found
 */
export async function getSoundByID(id: string): Promise<SoundModel | null> {
  const sounds = await getSoundsByIDs([id]);
  if (sounds.length < 1) return null;
  return sounds[0];
}

export function getSoundsByIDs(ids: string[]): Promise<SoundModel[]> {
  return Deno.core.ops.op_vtftk_get_sounds_by_ids(ids);
}

/**
 * Item stored within VTFTK
 *
 * @member id Unique ID of the item
 * @member name Name of the item
 * @member image Configuration for the item image
 * @member order Order of the item within the UI
 * @member created_at Date time when the item was created
 * @member impact_sound_ids List of IDs for sounds this item can play on impact
 */
export interface ItemModel {
  id: string;
  name: string;
  image: ItemModelImage;
  order: number;
  created_at: string;
  impact_sound_ids: string[];
}

/**
 * Item image config
 *
 * @member src URL for the image source
 * @member weight Weight the item has on impact (Affects how much the model flinches, Default: 1)
 * @member scale Scale of the image (Default: 1)
 * @member pixelate Whether to pixelate the image when scaling (Use to make pixel art scale properly)
 *
 */
export interface ItemModelImage {
  src: string;
  weight: number;
  scale: number;
  pixelate: boolean;
}

export interface ItemsWithSounds {
  items: ItemModel[];
  impact_sounds: SoundModel[];
}

type ThrowItemConfig =
  | { type: "All"; amount: number }
  | {
      type: "Barrage";
      amount_per_throw: number;
      amount: number;
      frequency: number;
    };

/**
 * Loads all the impact sounds for the
 * provided collection of items
 *
 * @param items The items
 * @returns The items with sounds
 */
export async function getItemsWithSounds(
  items: ItemModel[],
): Promise<ItemsWithSounds> {
  // Collect impact sound IDs
  const impactSoundIds = new Set<string>();
  for (const item of items) {
    for (const soundId of item.impact_sound_ids) {
      impactSoundIds.add(soundId);
    }
  }

  // Load impact sounds
  const sounds = await getSoundsByIDs(Array.from(impactSoundIds));
  return {
    items,
    impact_sounds: sounds,
  };
}

/**
 * Find an item by name
 *
 * @param name The name of the item
 * @param ignoreCase Whether to ignore case when searching
 * @returns The item if found otherwise null
 */
export async function getItemByName(
  name: string,
  ignoreCase: boolean = false,
): Promise<ItemModel | null> {
  const items = await getItemsByName(name, ignoreCase);
  if (items.length < 1) return null;
  return items[0];
}

/**
 * Find a collection of items by name
 *
 * @param names The name to search for
 * @param ignoreCase Whether to ignore case when searching
 * @returns The list of items found
 */
export function getItemsByName(
  name: string,
  ignoreCase: boolean = false,
): Promise<ItemModel[]> {
  return getItemsByNames([name], ignoreCase);
}

/**
 * Find a collection of items by names
 *
 * @param names The list of names to search for
 * @param ignoreCase Whether to ignore case when searching
 * @returns The list of items found
 */
export function getItemsByNames(
  names: string[],
  ignoreCase: boolean = false,
): Promise<ItemModel[]> {
  return Deno.core.ops.op_vtftk_get_items_by_names(names, ignoreCase);
}

/**
 * Find a specific item by ID
 *
 * @param id The ID of the item
 * @returns The found item or null if undefined
 */
export async function getItemById(id: string): Promise<ItemModel | null> {
  const items = await getItemsByIds([id]);
  if (items.length < 1) return null;
  return items[0];
}

/**
 * Finds a collection of items by their IDs
 *
 * @param ids The IDs of the items
 * @returns The items that were found
 */
export function getItemsByIds(ids: string[]): Promise<ItemModel[]> {
  return Deno.core.ops.op_vtftk_get_items_by_ids(ids);
}

/**
 * Throws a bunch of items all at once from a collection of IDs
 *
 * @param ids IDs of the items to throw
 * @param amount The total amount to throw all at once
 * @returns Promise resolved when the throw is queued
 */
export function throwAllByIds(
  ids: string[],
  amount: number = 10,
): Promise<void> {
  return throwItemsByIDs(ids, {
    type: "All",
    amount,
  });
}

/**
 * Throws a bunch of items all at once from a collection of names
 *
 * @param names The names of the items to throw
 * @param ignoreCase Whether to ignore casing when matching names
 * @param amount The total amount to throw all at once
 * @returns Promise resolved when the throw is queued
 */
export function throwAllByNames(
  names: string[],
  ignoreCase: boolean = false,
  amount: number = 10,
): Promise<void> {
  return throwItemsByNames(names, ignoreCase, {
    type: "All",
    amount,
  });
}

/**
 * Throws a bunch of items all at once
 *
 * @param items The items to throw
 * @param amount The total amount to throw all at once
 * @returns Promise resolved when the throw is queued
 */
export function throwAll(
  items: ItemsWithSounds,
  amount: number,
): Promise<void> {
  return throwItems(items, {
    type: "All",
    amount,
  });
}

/**
 * Configuration for how to throw a barrage
 *
 * @member totalAmount The total amount of items to throw
 * @member amountPerThrow The amount of items to throw per barrage
 * @member frequency The time between each barrage (ms)
 */
interface BarrageConfig {
  totalAmount?: number;
  amountPerThrow?: number;
  frequency?: number;
}

/**
 * Throw a barrage of items
 *
 * @param items The items with sounds to throw
 * @param config Configuration for how to throw the barrage
 * @returns Promise resolved when the throw is queued
 */
export function throwBarrage(
  items: ItemsWithSounds,
  config: BarrageConfig = {},
): Promise<void> {
  return throwItems(items, {
    type: "Barrage",
    amount: config.totalAmount ?? 15,
    amount_per_throw: config.amountPerThrow ?? 5,
    frequency: config.frequency ?? 100,
  });
}

/**
 * Throw a barrage of items by the IDs of the items
 *
 * @param ids The IDs of the items to throw
 * @param config Configuration for how to throw the barrage
 * @returns Promise resolved when the throw is queued
 */
export function throwBarrageByIds(
  ids: string[],
  config: BarrageConfig = {},
): Promise<void> {
  return throwItemsByIDs(ids, {
    type: "Barrage",
    amount: config.totalAmount ?? 15,
    amount_per_throw: config.amountPerThrow ?? 5,
    frequency: config.frequency ?? 100,
  });
}

/**
 * Throw a barrage of items by the names of the items
 *
 * @param names The names of the items
 * @param ignoreCase Whether to ignore casing when searching for names
 * @param config Configuration for how to throw the barrage
 * @returns Promise resolved when the throw is queued
 */
export function throwBarrageByNames(
  names: string[],
  ignoreCase: boolean = false,
  config: BarrageConfig = {},
): Promise<void> {
  return throwItemsByNames(names, ignoreCase, {
    type: "Barrage",
    amount: config.totalAmount ?? 15,
    amount_per_throw: config.amountPerThrow ?? 5,
    frequency: config.frequency ?? 100,
  });
}

/**
 * Throws a collection of items by ID
 *
 * @param ids The IDs of the items to throw
 * @param config Configuration for the throw
 * @returns Promise resolved when the throw is queued
 */
export async function throwItemsByIDs(ids: string[], config: ThrowItemConfig) {
  const items = await getItemsByIds(ids);
  const itemsWithSounds = await getItemsWithSounds(items);

  return throwItems(itemsWithSounds, config);
}

/**
 * Throws a collection of items by name
 *
 * @param names The names of the items to throw
 * @param ignoreCase  Whether to ignore casing when finding the items
 * @param config Configuration for the throw
 * @returns Promise resolved when the throw is queued
 */
export async function throwItemsByNames(
  names: string[],
  ignoreCase: boolean,
  config: ThrowItemConfig,
): Promise<void> {
  const items = await getItemsByNames(names, ignoreCase);
  const itemsWithSounds = await getItemsWithSounds(items);

  return throwItems(itemsWithSounds, config);
}

/**
 * Throws a collection of items
 *
 * @param items The items to throw
 * @param config Configuration for the throw
 * @returns Promise resolved when the throw is queued
 */
export function throwItems(
  items: ItemsWithSounds,
  config: ThrowItemConfig,
): Promise<void> {
  return Deno.core.ops.op_vtftk_throw_items(items, config);
}

/**
 * Trigger a VTube studio hotkey using its ID
 *
 * @param hotkeyID The ID of the hotkey to trigger
 * @returns Promise resolved when the hotkey is triggered
 */
export function triggerVTHotkey(hotkeyID: string) {
  return Deno.core.ops.op_vtftk_trigger_vt_hotkey(hotkeyID);
}

/**
 * Trigger a VTube studio hotkey using its name
 *
 * @param hotkeyName The name of the hotkey
 * @param ignoreCase Whether to ignore the case of the name
 * @returns Promise resolved when the hotkey is triggered
 */
export function triggerVTHotkeyByName(
  hotkeyName: string,
  ignoreCase: boolean = false,
) {
  return Deno.core.ops.op_vtftk_trigger_vt_hotkey_by_name(
    hotkeyName,
    ignoreCase,
  );
}
