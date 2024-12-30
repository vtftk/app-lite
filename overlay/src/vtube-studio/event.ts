/* eslint-disable @typescript-eslint/no-explicit-any */
import { createVTubeMessage } from "./message";
import { VTubeStudioWebSocket } from "./socket";

export async function subscribeEvent(
  socket: VTubeStudioWebSocket,
  eventName: string,
  subscribe: boolean = true,
  config: any = {},
): Promise<any> {
  const request = createVTubeMessage("EventSubscriptionRequest", {
    eventName,
    subscribe,
    config,
  });
  const response = await socket.send(request);

  if (!response.data) {
    throw new Error("missing response data");
  }

  return response.data.availableHotkeys;
}
