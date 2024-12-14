<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import {
    FileType,
    type ItemWithImpactSounds,
    type ThrowableImageConfig,
  } from "$lib/api/types";
  import { getRuntimeAppData } from "$lib/api/runtimeAppData";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import { goto } from "$app/navigation";
  import SoundPicker from "$lib/components/sounds/SoundPicker.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import ImageUpload from "$lib/components/form/ImageUpload.svelte";
  import { uploadFile } from "$lib/api/data";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import { toast } from "svelte-sonner";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";
  import FormBoundCheckbox from "$lib/components/form/FormBoundCheckbox.svelte";
  import { derived } from "svelte/store";
  import { createItemMutation, updateItemMutation } from "$lib/api/items";
  import { toastErrorMessage } from "$lib/utils/error";
  import { Tabs } from "bits-ui";
  import SolarAlbumBoldDuotone from "~icons/solar/album-bold-duotone";
  import SolarBookBoldDuotone from "~icons/solar/book-bold-duotone";
  import SolarVolumeLoudBoldDuotone from "~icons/solar/volume-loud-bold-duotone";

  type Props = {
    existing?: ItemWithImpactSounds;
  };

  const { existing }: Props = $props();

  const runtimeAppData = getRuntimeAppData();

  const updateItem = updateItemMutation();
  const createItem = createItemMutation();

  // Testing is only available when an overlay and vtube studio is connected
  const testingEnabled = derived(
    runtimeAppData,
    ($runtimeAppData) =>
      $runtimeAppData.active_overlay_count > 0 &&
      $runtimeAppData.vtube_studio_connected
  );

  // When working with existing configs we allow the file to be a
  // string to account for already uploaded file URLs
  const imageSchema = z
    .instanceof(File, {
      message: "Image file is required",
      fatal: true,
    })
    .or(z.string());

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    image: imageSchema,
    scale: z.number(),
    weight: z.number(),
    pixelate: z.boolean(),
    impactSoundIds: z.array(z.string()),
  });

  type Schema = z.infer<typeof schema>;

  // Defaults when creating a new throwable
  const createDefaults: Partial<Schema> = {
    name: "",
    image: undefined,
    scale: 1,
    weight: 1,
    pixelate: false,
    impactSoundIds: [],
  };

  function createFromExisting(config: ItemWithImpactSounds): Partial<Schema> {
    return {
      name: config.name,
      image: config.image.src,
      scale: config.image.scale,
      weight: config.image.weight,
      pixelate: config.image.pixelate,
      impactSoundIds: config.impact_sounds.map((sound) => sound.id),
    };
  }

  const { form, data, touched } = createForm<Schema>({
    // Derive initial values
    initialValues: existing ? createFromExisting(existing) : createDefaults,

    // Validation and error reporting
    extend: [validator({ schema }), reporterDom()],

    async onSubmit(values) {
      const savePromise = save(values);

      toast.promise(
        savePromise,
        existing
          ? {
              loading: "Saving item...",
              success: "Saved item",
              error: toastErrorMessage("Failed to save item"),
            }
          : {
              loading: "Creating item...",
              success: "Created item",
              error: toastErrorMessage("Failed to create item"),
            }
      );

      // Go back to the list when creating rather than editing
      if (!existing) {
        goto("/throwables");
      }
    },
  });

  // Store initial impact sounds list for checking touched state
  const initialImpactSoundIds = $data.impactSoundIds;

  // Touched state for impact sound IDs must be manually updated
  $effect(() => {
    if (initialImpactSoundIds !== $data.impactSoundIds) {
      $touched.impactSoundIds = true;
    }
  });

  function saveImage(image: string | File) {
    if (image instanceof File) {
      // Upload new image
      return uploadFile(FileType.ThrowableImage, image);
    }

    // Using existing uploaded image
    return Promise.resolve(image);
  }

  async function save(values: Schema) {
    const imageURL: string = await saveImage(values.image);
    const imageConfig: ThrowableImageConfig = {
      src: imageURL,
      pixelate: values.pixelate,
      scale: values.scale,
      weight: values.weight,
    };

    if (existing) {
      await $updateItem.mutateAsync({
        itemId: existing.id,
        update: {
          name: values.name,
          image: imageConfig,
          impact_sounds: values.impactSoundIds,
        },
      });
    } else {
      await $createItem.mutateAsync({
        name: values.name,
        image: imageConfig,
        impact_sounds: values.impactSoundIds,
      });
    }
  }

  function onTestThrow() {
    if (existing === undefined) return;

    const throwPromise = testThrow([existing.id], 1);

    toast.promise(throwPromise, {
      loading: "Sending throw...",
      success: "Threw item",
      error: toastErrorMessage("Failed to throw item"),
    });
  }

  function onTestBarrage() {
    if (existing === undefined) return;

    const throwPromise = testThrowBarrage([existing.id], 20, 2, 100);

    toast.promise(throwPromise, {
      loading: "Sending barrage...",
      success: "Threw barrage",
      error: toastErrorMessage("Failed to throw barrage"),
    });
  }
</script>

{#snippet actions()}
  {#if existing}
    <button
      type="button"
      class="btn"
      onclick={onTestThrow}
      disabled={!$testingEnabled}
    >
      <BallIcon /> Test
    </button>
    <button
      type="button"
      class="btn"
      onclick={onTestBarrage}
      disabled={!$testingEnabled}
    >
      <BallsIcon /> Test Barrage
    </button>
  {/if}
  <button type="submit" class="btn"> {existing ? "Save" : "Create"}</button>
  <a class="btn" href="/throwables">Back</a>
{/snippet}

<form use:form>
  <PageLayoutList
    title={existing ? "Edit Throwable" : "Create Throwable"}
    description={existing
      ? "Editing a throwable"
      : "Create a new item that can be thrown"}
    {actions}
  >
    <div class="content">
      <Tabs.Root>
        <Tabs.List>
          <Tabs.Trigger value="image">
            <SolarAlbumBoldDuotone />
            Image
          </Tabs.Trigger>
          <Tabs.Trigger value="details">
            <SolarBookBoldDuotone />
            Details
          </Tabs.Trigger>
          <Tabs.Trigger value="impact_sounds">
            <SolarVolumeLoudBoldDuotone />
            Impact Sounds
          </Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="details">
          <FormSection>
            <FormTextInput id="name" name="name" label="Name" />
          </FormSection>
        </Tabs.Content>
        <Tabs.Content value="image">
          <FormSection title="Image" description="Image that gets thrown">
            <div class="row-group">
              <ImageUpload
                id="image"
                name="image"
                label="Image"
                existing={existing?.image?.src}
                scale={$data.scale}
                pixelated={$data.pixelate}
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

                <FormBoundCheckbox
                  id="pixelate"
                  name="pixelate"
                  label="Pixelate"
                />
              </div>
            </div>
          </FormSection>
        </Tabs.Content>
        <Tabs.Content value="impact_sounds">
          <FormSection
            title="Impact Sounds"
            description="Choose selection of sounds that can play when the item impacts"
          >
            <SoundPicker bind:selected={$data.impactSoundIds} />
            <FormErrorLabel name="impactSoundIds" />
          </FormSection>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  </PageLayoutList>
</form>

<style>
  form {
    height: 100%;
  }

  .row-group {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .content {
    position: relative;
    flex: auto;
    overflow: hidden;
    height: 100%;
  }

  .content :global([data-tabs-root]) {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .content :global([data-tabs-content]) {
    position: relative;
    flex: auto;
    overflow: auto;
    flex-flow: column;
    border: 1px solid #333;
    padding: 1rem;
  }
</style>
