<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import { FileType, type Sound } from "$lib/api/types";
  import { goto } from "$app/navigation";
  import SoundUpload from "$lib/components/form/SoundUpload.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";
  import { uploadFile } from "$lib/api/data";
  import { toast } from "svelte-sonner";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import { createSoundMutation, updateSoundMutation } from "$lib/api/sounds";

  type Props = {
    existing?: Sound;
  };

  const { existing }: Props = $props();

  const updateSound = updateSoundMutation();
  const createSound = createSoundMutation();

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
                error: "Failed to save sound",
              }
            : {
                loading: "Creating sound...",
                success: "Created sound",
                error: "Failed to create sound",
              }
        );

        // Go back to the list when creating rather than editing
        if (!existing) {
          goto("/sounds");
        }
      },
    }
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
      await $updateSound.mutateAsync({
        soundId: existing.id,
        update: {
          src: soundURL,
          volume: values.volume,
          name: values.name,
        },
      });
    } else {
      await $createSound.mutateAsync({
        src: soundURL,
        volume: values.volume,
        name: values.name,
      });
    }
  }
</script>

<form use:form>
  {#snippet actions()}
    <button type="submit" class="btn" disabled={!$isValid}>
      {existing ? "Save" : "Create"}
    </button>

    <a class="btn" href="/sounds">Back</a>
  {/snippet}

  <PageLayoutList
    title={existing ? "Edit Sound" : "Create Sound"}
    description={existing
      ? "Editing a sound"
      : "Create a sound that can be triggered"}
    {actions}
  >
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
          volume={$data.volume}
        />
        <FormErrorLabel name="sound" />

        <FormNumberInput
          id="volume"
          name="volume"
          label="Volume"
          min={0}
          max={1}
          step={0.1}
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
