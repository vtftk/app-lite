const BACKEND_URL_PROD = "http://localhost:58371/";
const BACKEND_URL_DEV = "http://localhost:58372/";

export default function getBackendURL(url: string) {
  return url.replace(
    "backend://",
    import.meta.env.DEV ? BACKEND_URL_DEV : BACKEND_URL_PROD,
  );
}
