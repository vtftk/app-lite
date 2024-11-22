import { BACKEND_EVENTS } from "../constants";
import { VTubeStudioWebSocket } from "../vtube-studio/socket";
import { beginCalibrationStep } from "./calibration";

export function createEventSource(vtSocket: VTubeStudioWebSocket) {
  const eventSource = new EventSource(BACKEND_EVENTS);
  eventSource.onopen = () => {
    console.debug("listening to events");
  };
  eventSource.onmessage = (msg) => {
    console.log(msg);
    const event = JSON.parse(msg.data);

    if (event.type === "SetCalibrationStep") {
      beginCalibrationStep(vtSocket, event.step);
    }
  };

  eventSource.onerror = (event) => {
    console.error(event);
  };
}
