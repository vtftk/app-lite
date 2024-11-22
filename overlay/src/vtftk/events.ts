import { CalibrationStepData } from "./calibration-types";

function createEventSource() {
  const eventSource = new EventSource("http://localhost:58371/events");
  eventSource.onopen = () => {
    console.log("listening to events");
  };
  eventSource.onmessage = (event) => {
    console.log(event);
  };

  eventSource.onerror = (event) => {
    console.error(event);
  };
}

createEventSource();

export async function notifyProgressCalibration(body: CalibrationStepData) {
  await fetch(
    "http://localhost:58371/calibration",

    {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    }
  );
}
