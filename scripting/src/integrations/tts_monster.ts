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
 * @param token Access token for TTS monster
 * @returns The list of available voices
 */
export async function voices(token: string): Promise<TTSMonsterVoice[]> {
  const response = await api.http.post(
    "https://api.console.tts.monster/voices",
    {},
    {
      responseFormat: "json",
      headers: {
        authorization: token,
      },
    },
  );

  if (!response.ok)
    throw new Error(
      `Non 2xx response from TTS Monster /voices (${response.status}): ${response.body}`,
    );

  const body = response.body as { voices: TTSMonsterVoice[] };
  const voices = body.voices;

  return voices;
}

/**
 * Generate a single voice message using a specific voice
 *
 * @param token Access token for TTS monster
 * @param voice_id The ID of the voice to use
 * @param message The message for the voice to say
 * @returns URL to the voice message file
 */
export async function generate(
  token: string,
  voice_id: TTSMonsterVoiceId,
  message: string,
): Promise<string> {
  const response = await api.http.post(
    "https://api.console.tts.monster/generate",
    {
      voice_id,
      message,
    },
    {
      responseFormat: "json",
      headers: {
        authorization: token,
      },
    },
  );

  if (!response.ok)
    throw new Error(
      `Non 2xx response from TTS Monster /generate (${response.status}): ${response.body}`,
    );

  const body = response.body as { url: string };
  return body.url;
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
 * @param token Access token for TTS monster
 * @param message The message to parse and generate
 * @returns The list of URLs for each voice message segment
 */
export async function generateParsed(
  token: string,
  message: string,
): Promise<string[]> {
  const segments = parseTTSMessage(message);
  const voices = await api.integrations.tts_monster.voices(token);

  const defaultVoiceId = "a33aa2c5-47f9-4882-a192-d7aa6a0c0efd";
  const defaultVoice: TTSMonsterVoice | undefined = voices.find(
    (voice) => voice.voice_id === defaultVoiceId,
  );

  const voiceSegments: { voice: TTSMonsterVoice; message: string }[] = [];
  for (const segment of segments) {
    let voice: TTSMonsterVoice | undefined;
    if (segment.name) {
      const name = segment.name.toLowerCase().trim();
      const match = voices.find(
        (voice) => voice.name.toLowerCase().trim() === name,
      );
      voice = match ?? defaultVoice;
    } else {
      voice = defaultVoice;
    }

    // Skip unknown voice segment without default
    if (voice === undefined) continue;

    voiceSegments.push({
      voice,
      message: segment.message,
    });
  }

  const generated: string[] = [];

  const chunkSize = 5;

  for (let i = 0; i < voiceSegments.length; i += chunkSize) {
    const segments = voiceSegments.slice(i, i + chunkSize);
    const results = await Promise.allSettled(
      segments.map((segment) =>
        generate(token, segment.voice.voice_id, segment.message),
      ),
    );

    for (const result of results) {
      if (result.status !== "fulfilled") {
        console.error("failed to generated voice message", result.reason);
        continue;
      }

      generated.push(result.value);
    }
  }

  return generated;
}

type TTSMessageSegment = {
  name?: string;
  message: string;
};

function parseTTSMessage(message: string): TTSMessageSegment[] {
  const segments: TTSMessageSegment[] = [];
  let index = 0;

  while (index < message.length) {
    const nameResult = nextMessageName(message, index);
    if (nameResult.result === "EndOfInput") {
      break;
    }

    let name: string | undefined;
    if (nameResult.result === "UnknownSpeaker") {
      index = nameResult.index;
    } else {
      name = nameResult.name;
      index = nameResult.index;
    }

    // Collect message until next '('
    const messageStart = index;
    let messageEnd = messageStart;
    while (index < message.length && message[index] !== "(") {
      messageEnd++;
      index++;
    }
    const msg = message.slice(messageStart, messageEnd).trim();

    if (msg) {
      segments.push({
        name,
        message: msg,
      });
    }
  }

  return segments;
}

function nextMessageName(str: string, startIndex: number) {
  let i = startIndex;

  while (true) {
    // Skip leading whitespace
    while (i < str.length && str[i].trim() === "") {
      i++;
    }

    if (i >= str.length) {
      return { result: "EndOfInput", index: i };
    }

    if (str[i] !== "(") {
      return { result: "UnknownSpeaker", index: i };
    }

    // Consume '('
    i++;
    const nameStart = i;

    // Find closing ')'
    while (i < str.length && str[i] !== ")") {
      i++;
    }
    const name = str.slice(nameStart, i).trim();

    // Consume ')'
    if (i < str.length && str[i] === ")") {
      i++;
    }

    if (name === "") {
      // Continue searching for valid name
      continue;
    } else {
      return { result: "Name", name, index: i };
    }
  }
}
