import { createVTubeMessage } from "./message";
import { VTubeStudioWebSocket } from "./socket";

export type AvailableHotkey = {
  name: string;
  type: boolean;
  description: string;
  file: string;
  hotkeyID: string;
  keyCombination: any[];
  onScreenButtonID: number | null;
};

export async function requestHotkeys(
  socket: VTubeStudioWebSocket
): Promise<AvailableHotkey[]> {
  const request = createVTubeMessage("HotkeysInCurrentModelRequest", {});
  const response = await socket.send(request);

  if (!response.data) {
    throw new Error("missing response data");
  }

  return response.data.availableHotkeys;
}

export async function triggerHotkey(
  socket: VTubeStudioWebSocket,
  hotkeyID: string
): Promise<string> {
  const request = createVTubeMessage("HotkeyTriggerRequest", { hotkeyID });
  const response = await socket.send(request);

  if (!response.data) {
    throw new Error("missing response data");
  }

  return response.data.hotkeyID;
}
