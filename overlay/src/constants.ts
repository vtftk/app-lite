// Minimum model size
export const SMALLEST_MODEL_SIZE: number = -100;

// Maximum model size
export const LARGEST_MODEL_SIZE: number = 100;

// Total size range spanned by the model size
export const TOTAL_MODEL_SIZE_RANGE: number = Math.abs(
  SMALLEST_MODEL_SIZE - LARGEST_MODEL_SIZE
);

export const VTUBE_STUDIO_SOCKET_RETRY_TIMEOUT = 5 * 1000;

export const BACKEND_HTTP = getEndpoint();

function getEndpoint() {
  // Development override for the endpoint
  const envEndpoint = import.meta.env.VITE_API_ENDPOINT;
  if (envEndpoint !== undefined) {
    return envEndpoint;
  }

  // In production we use the Origin since the overlay is served from the API
  return window.location.origin;
}
