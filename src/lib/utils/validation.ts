import { z } from "zod";

export const minMax = z.object({
  min: z.number(),
  max: z.number(),
});
