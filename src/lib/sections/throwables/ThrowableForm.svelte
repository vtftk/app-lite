<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import type {
    ImpactSoundConfig,
    ItemConfig,
    ThrowableImageConfig,
  } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import { goto } from "$app/navigation";

  type Props = {
    existing?: ItemConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();

  const appDataMutation = createAppDateMutation();

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    image: existing
      ? z.union([z.instanceof(File), z.undefined()])
      : z.instanceof(File, {
          message: "Image file is required",
          fatal: true,
        }),
    scale: z.number(),
    weight: z.number(),
    pixelate: z.boolean(),
  });

  type Schema = z.infer<typeof schema>;

  const { form, data } = createForm<Schema>({
    initialValues: (existing
      ? {
          name: existing.name,
          image: undefined,
          scale: existing.image.scale,
          weight: existing.image.weight,
          pixelate: existing.image.pixelate,
        }
      : {
          name: "",
          image: undefined,
          scale: 1,
          weight: 1,
          pixelate: false,
        }) satisfies Schema,
    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      let imageURL: string;

      if (values.image) {
        imageURL = await invoke<string>("upload_file", {
          fileType: "ThrowableImage",
          fileName: values.image.name,
          fileData: await values.image.arrayBuffer(),
        });
      } else if (existing) {
        imageURL = existing.image.src;
      } else {
        throw new Error("image was missing in create mode");
      }

      const imageConfig: ThrowableImageConfig = {
        src: imageURL,
        pixelate: values.pixelate,
        scale: values.scale,
        weight: values.weight,
      };

      const throwableConfig: ItemConfig = {
        id: existing ? existing.id : self.crypto.randomUUID(),
        image: imageConfig,
        impact_sounds_ids: [],
        name: values.name,
      };

      if (existing) {
        await $appDataMutation.mutateAsync({
          ...$appData,
          items: $appData.items.map((item) => {
            if (item.id !== existing.id) return item;
            return throwableConfig;
          }),
        });
      } else {
        await $appDataMutation.mutateAsync({
          ...$appData,
          items: [...$appData.items, throwableConfig],
        });
      }

      goto("/throwables");
    },
  });
</script>

<form use:form>
  <div>
    <label for="name">Name</label>
    <input
      type="text"
      id="name"
      name="name"
      aria-describedby="name-validation"
    />
    <FormErrorLabel name="name" />
  </div>

  <div>
    <h2>Image</h2>
    <p>Image that gets thrown at the model</p>

    {#if existing}
      <div>
        <img
          src={existing.image.src}
          style="transform: scale({$data.scale})"
          alt=""
        />
      </div>
    {/if}

    <div>
      <label for="image">{existing ? "Replace" : "Upload"} Image</label>
      <input
        accept="image/*"
        type="file"
        id="image"
        name="image"
        aria-describedby="image-validation"
      />
      <FormErrorLabel name="image" />
    </div>

    <div>
      <label for="scale">Scale</label>
      <input
        type="number"
        id="scale"
        name="scale"
        min="0.1"
        max="1"
        step="0.1"
        aria-describedby="scale-validation"
      />
      <FormErrorLabel name="scale" />
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
      <FormErrorLabel name="weight" />
    </div>

    <div>
      <label for="pixelate">Pixelate</label>
      <input
        type="checkbox"
        id="pixelate"
        name="pixelate"
        aria-describedby="pixelate-validation"
      />
      <FormErrorLabel name="pixelate" />
    </div>
  </div>

  <div>
    <h2>Impact Sounds</h2>
    <p>
      Sound played when the throwable impacts
      <span>Optional</span>
    </p>
  </div>

  <button type="submit"> {existing ? "Save" : "Create"}</button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
