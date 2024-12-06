<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import type { SoundConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";
  import SoundUpload from "$lib/components/form/SoundUpload.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";

  type Props = {
    existing?: SoundConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    sound:
      // Allow not specifying file when updating existing
      existing !== undefined
        ? z.union([z.instanceof(File), z.undefined()])
        : z.instanceof(File, {
            message: "Sound file is required",
            fatal: true,
          }),
    volume: z.number(),
  });

  const { form, data, isValid, setFields } = createForm<z.infer<typeof schema>>(
    {
      initialValues: existing
        ? {
            name: existing.name,
            sound: undefined,
            volume: existing.volume,
          }
        : {
            name: "",
            sound: undefined,
            volume: 1,
          },
      extend: [validator({ schema }), reporterDom()],
      async onSubmit(values, context) {
        let soundURL: string;

        if (values.sound) {
          soundURL = await invoke<string>("upload_file", {
            fileType: "Sound",
            fileName: values.sound.name,
            fileData: await values.sound.arrayBuffer(),
          });
        } else if (existing) {
          soundURL = existing.src;
        } else {
          throw new Error("sound was missing in create mode");
        }

        const soundConfig: SoundConfig = {
          id: existing ? existing.id : self.crypto.randomUUID(),
          src: soundURL,
          volume: values.volume,
          name: values.name,
        };

        if (existing !== undefined) {
          // Update existing
          await $appDataMutation.mutateAsync({
            ...$appData,
            sounds: $appData.sounds.map((item) => {
              if (item.id !== existing.id) return item;
              return soundConfig;
            }),
          });
        } else {
          // Add new
          await $appDataMutation.mutateAsync({
            ...$appData,
            sounds: [...$appData.sounds, soundConfig],
          });
        }

        goto("/sounds");
      },
    }
  );
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
