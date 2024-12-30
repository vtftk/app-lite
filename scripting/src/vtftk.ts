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
