<script lang="ts">
  import { z } from "zod";
  import { createForm } from "felte";
  import { toast } from "svelte-sonner";
  import { goto } from "$app/navigation";
  import { uploadFile } from "$lib/api/data";
  import reporterDom from "@felte/reporter-dom";
  import { validator } from "@felte/validator-zod";
  import { toastErrorMessage } from "$lib/utils/error";
  import { getAppData } from "$lib/api/runtimeAppData";
  import { FileType, type Sound } from "$lib/api/types";
  import Button from "$lib/components/input/Button.svelte";
  import { createSound, updateSound } from "$lib/api/soundModel";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSlider from "$lib/components/form/FormSlider.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import SoundUpload from "$lib/components/form/SoundUpload.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";
  import SolarAltArrowLeftBold from "~icons/solar/alt-arrow-left-bold";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";

  type Props = {
    existing?: Sound;
  };

  const appData = getAppData();

  const { existing }: Props = $props();

  // When working with existing configs we allow the file to be a
  // string to account for already uploaded file URLs
  const soundSchema = z
    .instanceof(File, {
      message: "Sound file is required",
      fatal: true,
    })
    .or(z.string());

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    sound: soundSchema,
    volume: z.number(),
  });

  type Schema = z.infer<typeof schema>;

  // Defaults when creating a new sound
  const createDefaults: Partial<Schema> = {
    name: "",
    sound: undefined,
    volume: 1,
  };

  function createFromExisting(config: Sound): Partial<Schema> {
    return {
      name: config.name,
      sound: config.src,
      volume: config.volume,
    };
  }

  const { form, data, isValid, setFields } = createForm<z.infer<typeof schema>>(
    {
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
                loading: "Saving sound...",
                success: "Saved sound",
                error: toastErrorMessage("Failed to save sound"),
              }
            : {
                loading: "Creating sound...",
                success: "Created sound",
                error: toastErrorMessage("Failed to create sound"),
              },
        );

        // Go back to the list when creating rather than editing
        if (!existing) {
          goto("/sounds");
        }
      },
    },
  );

  function saveSound(sound: string | File) {
    if (sound instanceof File) {
      // Upload new sound
      return uploadFile(FileType.Sound, sound);
    }

    // Using existing uploaded sound
    return Promise.resolve(sound);
  }

  async function save(values: Schema) {
    const soundURL: string = await saveSound(values.sound);

    if (existing !== undefined) {
      await updateSound({
        soundId: existing.id,
        update: {
          src: soundURL,
          volume: values.volume,
          name: values.name,
        },
      });
    } else {
      await createSound({
        src: soundURL,
        volume: values.volume,
        name: values.name,
      });
    }
  }
</script>

<form use:form>
  <PageLayoutList
    title={existing ? "Edit Sound" : "Create Sound"}
    description={existing
      ? `Editing "${existing.name}"`
      : "Create a sound that can be triggered"}
  >
    <!-- Back button -->
    {#snippet beforeTitle()}
      <LinkButton href="/sounds">
        <SolarAltArrowLeftBold />
      </LinkButton>
    {/snippet}

    <!-- End actions -->
    {#snippet actions()}
      <Button type="submit" disabled={!$isValid}>
        {existing ? "Save" : "Create"}
      </Button>
    {/snippet}

    <FormSections>
      <FormSection>
        <FormTextInput id="name" name="name" label="Name" />
      </FormSection>

      <FormSection>
        <SoundUpload
          id="sound"
          name="sound"
          label="Sound"
          existing={existing?.src}
          onChangeSound={(file) => {
            // Use the file name if the name hasn't been touched yet
            if ($data.name.length < 1 && file) {
              setFields("name", file.name);
            }
          }}
          volume={$data.volume * $appData.sounds_config.global_volume}
        />
        <FormErrorLabel name="sound" />

        <FormSlider
          id="volume"
          name="volume"
          label="Volume"
          description="Base volume the sound is played out"
          min={0}
          max={1}
          step={0.1}
          value={$data.volume}
          showTicks
        />
      </FormSection>
    </FormSections>
  </PageLayoutList>
</form>

<style>
  form {
    height: 100%;
  }
</style>
