<script lang="ts">
  import { bulkCreateItemMutation } from "$lib/api/items";
  import type { ThrowableImageConfig } from "$shared/appData";
  import type { CreateItem } from "$shared/dataV2";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "svelte-sonner";

  const bulkCreateItem = bulkCreateItemMutation();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeImage() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const images = Array.from(files);

    const createItems = await Promise.all(
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

        const createItem: CreateItem = {
          image: imageConfig,
          name: image.name,
          impact_sounds: [],
        };

        return createItem;
      })
    );

    const createPromise = $bulkCreateItem.mutateAsync(createItems);

    toast.promise(createPromise, {
      loading: "Creating items...",
      success: "Created items",
      error: "Failed to create items",
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
