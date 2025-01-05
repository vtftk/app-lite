export type TTSMonsterVoiceId = string;

export interface TTSMonsterVoice {
  // Voice ID to use when generating
  voice_id: TTSMonsterVoiceId;
  name: string;
  sample: string;
}

/**
 * Requests the list of voices from TTS Monster
 *
 * @returns The list of available voices
 */
export function voices(): Promise<TTSMonsterVoice[]> {
  return Deno.core.ops.op_vtftk_tts_get_voices();
}

/**
 * Generate a single voice message using a specific voice
 *
 * @param voice_id The ID of the voice to use
 * @param message The message for the voice to say
 * @returns URL to the voice message file
 */
export function generate(
  voice_id: TTSMonsterVoiceId,
  message: string
): Promise<string> {
  return Deno.core.ops.op_vtftk_tts_generate(voice_id, message);
}

/**
 * Generates a TTS voices uses the names and messages parsed from the
 * provided message i.e
 *
 *  "(Name1) This is the message for Name1 (Name2) This is the message for Name2"
 *
 * This will create voice messages for each of the voices returning the
 * messages in order
 *
 * @param message The message to parse and generate
 * @returns The list of URLs for each voice message segment
 */
export function generateParsed(message: string): Promise<string[]> {
  return Deno.core.ops.op_vtftk_tts_generate_parsed(message);
}
