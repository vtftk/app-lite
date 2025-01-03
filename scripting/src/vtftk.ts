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
  return Deno.core.ops.op_vtftk_get_sounds_by_name(name, ignoreCase);
}

/**
 * Collects a list of sounds by names from a list of names
 *
 * @param names The names to find
 * @param ignoreCase Whether to ignore name casing when searching
 * @returns Promise resolved to the list of sounds found
 */
export async function getSoundsByNames(
  names: string[],
  ignoreCase: boolean = false,
): Promise<SoundModel[]> {
  const results = await Promise.allSettled(
    names.map(async (name) => {
      try {
        return await getSoundsByName(name, ignoreCase);
      } catch (error) {
        console.error("error loading sounds by name", { name, error });
        throw error;
      }
    }),
  );
  const sounds: SoundModel[] = [];

  for (const result of results) {
    if (result.status === "fulfilled") {
      sounds.push(...result.value);
    }
  }

  return sounds;
}

/**
 * Find a sound using its unique ID
 *
 * @param id The ID of the sound
 * @returns The sound that was found or null if none were found
 */
export function getSoundByID(id: string): Promise<SoundModel | null> {
  return Deno.core.ops.op_vtftk_get_sound_by_id(id);
}

/**
 * Gets a collection of sounds using a list of IDs for
 * the sounds to find
 *
 * Sounds that were not found will not be included in the
 * result and will be logged as an error
 *
 * @param ids The IDs of the sounds to find
 * @returns The list of sounds
 */
export async function getSoundsByIDs(ids: string[]): Promise<SoundModel[]> {
  const results = await Promise.allSettled(
    ids.map(async (id) => {
      try {
        const sound = await getSoundByID(id);
        if (sound === null) throw new Error("sound does not exist");
        return sound;
      } catch (error) {
        console.error("error loading sound", { id, error });
        throw error;
      }
    }),
  );
  const sounds: SoundModel[] = [];

  for (const result of results) {
    if (result.status === "fulfilled") {
      sounds.push(result.value);
    }
  }

  return sounds;
}
