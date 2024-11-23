<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import { scale } from "svelte/transition";
  import type { ThrowableConfig } from "$lib/api/types";

  type Props = {
    onClose: VoidFunction;
  };

  const { onClose }: Props = $props();

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    image: z.instanceof(File, {
      message: "Image file is required",
      fatal: true,
    }),
    scale: z.number(),
    weight: z.number(),
    pixelate: z.boolean(),
    sound: z.union([z.instanceof(File), z.undefined()]),
    volume: z.number(),
  });

  const { form } = createForm<z.infer<typeof schema>>({
    initialValues: {
      name: "",
      image: undefined,
      scale: 1,
      weight: 1,
      pixelate: false,
      sound: undefined,
      volume: 1,
    },
    extend: [validator({ schema }), reporterDom()],
    onSubmit(values, context) {
      const throwableConfig: ThrowableConfig = {};

      console.log("submitted", values);
    },
  });
</script>

<button type="button" onclick={onClose}>Back</button>

<form use:form>
  <h1>Create Throwable</h1>
  <div>
    <label for="name">Name</label>
    <input
      type="text"
      id="name"
      name="name"
      aria-describedby="name-validation"
    />
    <p
      id="name-validation"
      data-felte-reporter-dom-for="name"
      aria-live="polite"
    ></p>
  </div>

  <div>
    <h2>Image</h2>
    <p>Image that gets thrown at the model</p>

    <div>
      <label for="image">Upload Image</label>
      <input
        accept="image/*"
        type="file"
        id="image"
        name="image"
        aria-describedby="image-validation"
      />
      <p
        id="image-validation"
        data-felte-reporter-dom-for="image"
        aria-live="polite"
      ></p>
    </div>

    <div>
      <label for="Scale">Scale</label>
      <input
        type="number"
        id="scale"
        name="scale"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="scale-validation"
      />
      <p
        id="scale-validation"
        data-felte-reporter-dom-for="scale"
        aria-live="polite"
      ></p>
    </div>

    <div>
      <label for="weight">Weight</label>
      <input
        type="number"
        id="weight"
        name="weight"
        min="0"
        max="10"
        step="0.1"
        aria-describedby="weight-validation"
      />
      <p
        id="weight-validation"
        data-felte-reporter-dom-for="weight"
        aria-live="polite"
      ></p>
    </div>

    <div>
      <label for="pixelate">Pixelate</label>
      <input
        type="checkbox"
        id="pixelate"
        name="pixelate"
        aria-describedby="pixelate-validation"
      />
      <p
        id="pixelate-validation"
        data-felte-reporter-dom-for="pixelate"
        aria-live="polite"
      ></p>
    </div>
  </div>

  <div>
    <h2>Impact Sound</h2>
    <p>
      Sound played when the throwable impacts
      <span>Optional</span>
    </p>

    <div>
      <label for="sound">Upload Sound</label>
      <input
        accept="audio/*"
        type="file"
        id="sound"
        name="sound"
        aria-describedby="sound-validation"
      />
      <p
        id="sound-validation"
        data-felte-reporter-dom-for="sound"
        aria-live="polite"
      ></p>
    </div>

    <div>
      <label for="volume">Volume</label>
      <input
        type="number"
        id="volume"
        name="volume"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="volume-validation"
      />
      <p
        id="volume-validation"
        data-felte-reporter-dom-for="volume"
        aria-live="polite"
      ></p>
    </div>
  </div>

  <button type="submit">Submit</button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
