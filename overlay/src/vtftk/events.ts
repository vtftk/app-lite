import { beginCalibrationStep } from "./calibration";
import { CalibrationStepData } from "./calibration-types";

function createEventSource() {
  const eventSource = new EventSource("http://localhost:58371/events");
  eventSource.onopen = () => {
    console.log("listening to events");
  };
  eventSource.onmessage = (msg) => {
    console.log(msg);
    const event = JSON.parse(msg.data);

    if (event.type === "SetCalibrationStep") {
      beginCalibrationStep(event.step);
    }
  };

  eventSource.onerror = (event) => {
    console.error(event);
  };
}

createEventSource();
