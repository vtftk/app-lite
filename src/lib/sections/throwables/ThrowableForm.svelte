<script lang="ts">
  import { createForm } from "felte";
  import { toast } from "svelte-sonner";
  import { goto } from "$app/navigation";
  import { uploadFile } from "$lib/api/data";
  import { validator } from "@felte/validator-zod";
  import HTabs from "$lib/components/HTabs.svelte";
  import { reporter } from "@felte/reporter-svelte";
  import { toastErrorMessage } from "$lib/utils/error";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import { getAppContext } from "$lib/api/runtimeAppData";
  import Button from "$lib/components/input/Button.svelte";
  import { createItem, updateItem } from "$lib/api/itemModel";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSlider from "$lib/components/form/FormSlider.svelte";
  import { itemSchema, type ItemSchema } from "$lib/schemas/item";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import ImageUpload from "$lib/components/form/ImageUpload.svelte";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import SoundPicker from "$lib/components/sounds/SoundPicker.svelte";
  import SolarAltArrowLeftBold from "~icons/solar/alt-arrow-left-bold";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SelectedSounds from "$lib/components/sounds/SelectedSounds.svelte";
  import FormBoundCheckbox from "$lib/components/form/FormBoundCheckbox.svelte";
  import SolarGalleryRoundBoldDuotone from "~icons/solar/gallery-round-bold-duotone";
  import SolarHeadphonesRoundBoldDuotone from "~icons/solar/headphones-round-bold-duotone";
  import {
    StorageFolder,
    type ItemConfig,
    type ItemImageConfig,
    type ItemWithImpactSounds,
  } from "$lib/api/types";

  type Props = {
    existing?: ItemWithImpactSounds;
  };
  const { existing }: Props = $props();

  const appContext = getAppContext();
  const runtimeAppData = $derived(appContext.runtimeAppData);

  // Testing is only available when an overlay and vtube studio is connected
  const testingEnabled = $derived(
    runtimeAppData.active_overlay_count > 0 &&
      runtimeAppData.vtube_studio_connected,
  );

  // Defaults when creating a new throwable
  const createDefaults: Partial<ItemSchema> = {
    name: "",
    image: undefined,
    scale: 1,
    weight: 1,
    pixelate: false,
    impactSoundIds: [],
  };

  function createFromExisting(
    config: ItemWithImpactSounds,
  ): Partial<ItemSchema> {
    const { image, windup } = config.config;
    return {
      name: config.name,
      image: image.src,
      scale: image.scale,
      weight: image.weight,
      pixelate: image.pixelate,
      impactSoundIds: config.impact_sounds,
    };
  }

  const { form, data, touched, setFields } = createForm<ItemSchema>({
    // Derive initial values
    initialValues: existing ? createFromExisting(existing) : createDefaults,

    // Validation and error reporting
    extend: [validator({ schema: itemSchema }), reporter()],

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
            },
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
      return uploadFile(StorageFolder.ThrowableImage, image);
    }

    // Using existing uploaded image
    return Promise.resolve(image);
  }

  async function save(values: ItemSchema) {
    const imageURL: string = await saveImage(values.image);
    const image: ItemImageConfig = {
      src: imageURL,
      pixelate: values.pixelate,
      scale: values.scale,
      weight: values.weight,
    };
    const config: ItemConfig = {
      image,
      windup: {
        enabled: false,
        duration: 0,
      },
    };

    if (existing) {
      await updateItem({
        itemId: existing.id,
        update: {
          name: values.name,
          config,
          impact_sounds: values.impactSoundIds,
          windup_sounds: [],
        },
      });
    } else {
      await createItem({
        name: values.name,
        config,
        impact_sounds: values.impactSoundIds,
        windup_sounds: [],
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

{#snippet detailsTab()}
  <FormSection>
    <FormTextInput
      id="name"
      name="name"
      label="Name"
      placeholder="Example Throwable..."
      minlength={1}
      required
    />

    <FormSlider
      id="weight"
      name="weight"
      label="Weight"
      min={0}
      max={4}
      step={0.1}
      value={$data.weight}
      description="Weight affects how much force your model is hit with when the item impacts (Default: 1)"
      showTicks
    />
  </FormSection>
{/snippet}

{#snippet imageTab()}
  <FormSection>
    <div class="row-group">
      <div class="column">
        <ImageUpload
          id="image"
          name="image"
          value={$data.image ?? existing?.config.image?.src}
          scale={$data.scale * 0.5}
          pixelated={$data.pixelate}
        />
        <FormErrorLabel name="image" />
      </div>

      <div class="column" style="flex: auto;">
        <FormNumberInput
          id="scale"
          name="scale"
          label="Scale"
          min={0.1}
          max={10}
          step={0.1}
        />

        <FormBoundCheckbox
          id="pixelate"
          name="pixelate"
          label="Pixelate"
          description="Use this option if your image is pixel art"
        />
      </div>
    </div>
  </FormSection>
{/snippet}

{#snippet soundsTab()}
  <FormSection
    title="Impact Sounds"
    description="Choose selection of sounds that can play when the item impacts"
  >
    <SoundPicker
      description="Choose which sounds should play when this item impacts"
      selected={$data.impactSoundIds}
      onChangeSelected={(soundIds) => {
        setFields(
          "impactSoundIds",
          soundIds.map((sound) => sound.id),
          true,
        );
      }}
    />

    {#if $data.impactSoundIds.length > 0}
      <SelectedSounds soundIds={$data.impactSoundIds} />
      <FormErrorLabel name="impactSoundIds" />
    {/if}
  </FormSection>
{/snippet}

<form use:form>
  <PageLayoutList
    title={existing ? "Edit Throwable" : "Create Throwable"}
    description={existing
      ? `Editing "${existing.name}"`
      : "Create a new item that can be thrown"}
  >
    <!-- Back button -->
    {#snippet beforeTitle()}
      <LinkButton href="/throwables">
        <SolarAltArrowLeftBold />
      </LinkButton>
    {/snippet}

    <!-- End actions -->
    {#snippet actions()}
      {#if existing}
        <!-- Button to test throwable -->
        <PopoverButton disabled={!testingEnabled}>
          {#snippet content()}
            <Button
              type="button"
              onclick={onTestThrow}
              disabled={!testingEnabled}
            >
              <BallIcon /> Test
            </Button>
            <Button
              type="button"
              onclick={onTestBarrage}
              disabled={!testingEnabled}
            >
              <BallsIcon /> Test Barrage
            </Button>
          {/snippet}

          <BallIcon /> Test
        </PopoverButton>
      {/if}
      <Button type="submit">{existing ? "Save" : "Create"}</Button>
    {/snippet}

    <HTabs
      tabs={[
        {
          value: "details",
          icon: SolarSettingsBoldDuotone,
          label: "Details",
          content: detailsTab,
        },
        {
          value: "image",
          icon: SolarGalleryRoundBoldDuotone,
          label: "Image",
          content: imageTab,
        },
        {
          value: "sounds",
          icon: SolarHeadphonesRoundBoldDuotone,
          label: "Sounds",
          content: soundsTab,
        },
      ]}
    />
  </PageLayoutList>
</form>

<style>
  .column {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }

  form {
    height: 100%;
  }

  .row-group {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
  }
</style>
