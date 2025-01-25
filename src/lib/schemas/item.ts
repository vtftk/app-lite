import { z } from "zod";

// When working with existing configs we allow the file to be a
// string to account for already uploaded file URLs
export const throwableImageSchema = z.union(
  [
    z.instanceof(File, {
      message: "Image file is required",
      fatal: true,
    }),
    z.string(),
  ],
  {
    message: "Image is required",
  },
);

export type ThrowableImageSchema = z.infer<typeof throwableImageSchema>;

export const itemSchema = z.object({
  name: z.string().min(1, "You must specify a name"),
  image: throwableImageSchema,
  scale: z.number(),
  weight: z.number(),
  pixelate: z.boolean(),
  impactSoundIds: z.array(z.string()),
});

export type ItemSchema = z.infer<typeof itemSchema>;
