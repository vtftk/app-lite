// Minimum model size
export const SMALLEST_MODEL_SIZE: number = -100;

// Maximum model size
export const LARGEST_MODEL_SIZE: number = 100;

// Total size range spanned by the model size
export const TOTAL_MODEL_SIZE_RANGE: number = Math.abs(
  SMALLEST_MODEL_SIZE - LARGEST_MODEL_SIZE
);

export const VTUBE_STUDIO_SOCKET_RETRY_TIMEOUT = 5 * 1000;

export const BACKEND_HTTP = "http://localhost:58371";
export const BACKEND_EVENTS = "http://localhost:58371/events";
