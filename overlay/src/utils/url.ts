import { BACKEND_URL } from "../constants";

export default function getBackendURL(url: string) {
  return url.replace("backend://", BACKEND_URL);
}
