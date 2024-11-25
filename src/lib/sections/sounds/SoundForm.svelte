<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import type { SoundConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";

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

  const { form } = createForm<z.infer<typeof schema>>({
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
    <p
      id="name-validation"
      data-felte-reporter-dom-for="name"
      aria-live="polite"
    ></p>
  </div>

  <div>
    <h2>Sound</h2>
    <p>Sound to play</p>

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

  <button type="submit">
    {existing ? "Save" : "Create"}
  </button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
