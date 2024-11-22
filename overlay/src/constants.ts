/// Minimum model size
export const SMALLEST_MODEL_SIZE: number = -100;

/// Maximum model size
export const LARGEST_MODEL_SIZE: number = 100;

/// Total size range spanned by the model size
export const TOTAL_MODEL_SIZE_RANGE: number = Math.abs(
  SMALLEST_MODEL_SIZE - LARGEST_MODEL_SIZE
);
