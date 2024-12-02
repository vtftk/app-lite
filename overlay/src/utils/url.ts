import { BACKEND_HTTP } from "../constants";

export default function getBackendURL(url: string) {
  return url.replace("backend://", BACKEND_HTTP);
}
