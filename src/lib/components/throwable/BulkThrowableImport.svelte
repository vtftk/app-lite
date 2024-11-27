<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import type {
    ItemConfig,
    SoundConfig,
    ThrowableImageConfig,
  } from "$shared/appData";
  import { invoke } from "@tauri-apps/api/core";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeImage() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const images = Array.from(files);

    const itemConfigs = await Promise.all(
      images.map(async (image) => {
        const imageURL = await invoke<string>("upload_file", {
          fileType: "ThrowableImage",
          fileName: image.name,
          fileData: await image.arrayBuffer(),
        });

        const imageConfig: ThrowableImageConfig = {
          src: imageURL,
          pixelate: false,
          scale: 1,
          weight: 1,
        };

        const itemConfig: ItemConfig = {
          id: self.crypto.randomUUID(),
          image: imageConfig,
          impact_sounds_ids: [],
          name: image.name,
        };

        return itemConfig;
      })
    );

    await $appDataMutation.mutateAsync({
      ...$appData,
      items: [...$appData.items, ...itemConfigs],
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
  Bulk Create Throwables
</button>

<input
  bind:this={inputElm}
  hidden
  multiple
  style="display: none;"
  type="file"
  onchange={onChangeImage}
  accept="image/*"
/>

<style>
</style>
