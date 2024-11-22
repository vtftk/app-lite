import { BACKEND_HTTP } from "../constants";
import { AppData } from "./types";

let appData: AppData | undefined;

export async function loadAppData() {
  const response = await fetch(new URL("/app-data", BACKEND_HTTP));
  const json: AppData = await response.json();
  return json;
}

export async function getAppData(): Promise<AppData> {
  if (!appData) {
    appData = await loadAppData();
  }
  return appData;
}
