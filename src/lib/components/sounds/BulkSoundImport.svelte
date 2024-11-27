<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import type { SoundConfig } from "$shared/appData";
  import { invoke } from "@tauri-apps/api/core";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeSound() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const sounds = Array.from(files);

    const soundConfigs = await Promise.all(
      sounds.map(async (sound) => {
        const soundURL = await invoke<string>("upload_file", {
          fileType: "Sound",
          fileName: sound.name,
          fileData: await sound.arrayBuffer(),
        });

        const soundConfig: SoundConfig = {
          id: self.crypto.randomUUID(),
          src: soundURL,
          volume: 1,
          name: sound.name,
        };

        return soundConfig;
      })
    );

    await $appDataMutation.mutateAsync({
      ...$appData,
      sounds: [...$appData.sounds, ...soundConfigs],
    });
  }
</script>

<button
  class="btn"
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  Bulk Create Sounds
</button>

<input
  bind:this={inputElm}
  hidden
  multiple
  style="display: none;"
  type="file"
  onchange={onChangeSound}
  accept="audio/*"
/>

<style>
</style>
