<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import {
    FileType,
    type ItemConfig,
    type ThrowableImageConfig,
  } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import {
    createAppDateMutation,
    createCreateItemMutation,
    createUpdateItemMutation,
    getAppData,
  } from "$lib/api/runtimeAppData";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import { goto } from "$app/navigation";
  import SoundPicker from "$lib/components/sounds/SoundPicker.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import FormCheckbox from "$lib/components/form/FormCheckbox.svelte";
  import ImageUpload from "$lib/components/form/ImageUpload.svelte";
  import { uploadFile } from "$lib/api/data";

  type Props = {
    existing?: ItemConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const updateItem = createUpdateItemMutation(appData, appDataMutation);
  const createItem = createCreateItemMutation(appData, appDataMutation);

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
    impactSoundIds: z.array(z.string()),
  });

  type Schema = z.infer<typeof schema>;

  const { form, data, touched, setFields } = createForm<Schema>({
    initialValues: (existing
      ? {
          name: existing.name,
          image: undefined,
          scale: existing.image.scale,
          weight: existing.image.weight,
          pixelate: existing.image.pixelate,
          impactSoundIds: existing.impact_sounds_ids,
        }
      : {
          name: "",
          image: undefined,
          scale: 1,
          weight: 1,
          pixelate: false,
          impactSoundIds: [],
        }) satisfies Schema,
    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      let imageURL: string;

      if (values.image) {
        imageURL = await uploadFile(FileType.ThrowableImage, values.image);
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

      if (existing) {
        const itemConfig: Omit<ItemConfig, "id"> = {
          image: imageConfig,
          impact_sounds_ids: values.impactSoundIds,
          name: values.name,
        };

        $updateItem({ itemId: existing.id, itemConfig });
      } else {
        const itemConfig: ItemConfig = {
          id: self.crypto.randomUUID(),
          image: imageConfig,
          impact_sounds_ids: values.impactSoundIds,
          name: values.name,
        };

        $createItem({ itemConfig });
      }

      goto("/throwables");
    },
  });

  const initialImpactSoundIds = $data.impactSoundIds;

  // Touched state for impact sound IDs must be manually updated
  $effect(() => {
    if (initialImpactSoundIds !== $data.impactSoundIds) {
      $touched.impactSoundIds = true;
    }
  });
</script>

<form use:form>
  <section class="section">
    <FormTextInput id="name" name="name" label="Name" />
  </section>

  <section class="section">
    <h2>Image</h2>
    <p>Image that gets thrown at the model</p>

    <ImageUpload
      id="image"
      name="image"
      label="Image"
      existing={existing?.image?.src}
      scale={$data.scale}
    />

    <div class="row-group">
      <FormNumberInput
        id="scale"
        name="scale"
        label="Scale"
        min={0.1}
        max={10}
        step={0.1}
      />

      <FormNumberInput
        id="weight"
        name="weight"
        label="Weight"
        min={0}
        max={10}
        step={0.1}
      />

      <FormCheckbox
        id="pixelate"
        name="pixelate"
        label="Pixelate"
        checked={$data.pixelate}
        onChecked={(checked) => {
          setFields("pixelate", checked, true);
        }}
      />
    </div>
  </section>

  <section class="section">
    <h2>Impact Sounds</h2>
    <p>
      Sound played when the throwable impacts
      <span>Optional</span>
    </p>

    <SoundPicker
      sounds={$appData.sounds}
      bind:selected={$data.impactSoundIds}
    />
    <FormErrorLabel name="impactSoundIds" />
  </section>

  <button type="submit" class="btn"> {existing ? "Save" : "Create"}</button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }

  .section {
    display: flex;
    flex-flow: column;

    border: 1px solid #333;
    padding: 1rem;
    gap: 1rem;
  }

  .row-group {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>
