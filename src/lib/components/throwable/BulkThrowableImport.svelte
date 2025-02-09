<script lang="ts">
  import type { CreateItem, ItemImageConfig } from "$lib/api/types";

  import { toast } from "svelte-sonner";
  import { uploadFile } from "$lib/api/data";
  import { StorageFolder } from "$lib/api/types";
  import { bulkCreateItem } from "$lib/api/itemModel";
  import { toastErrorMessage } from "$lib/utils/error";

  import Button from "../input/Button.svelte";

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeImage() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const images = Array.from(files);

    const createItems = await Promise.all(
      images.map(async (imageFile) => {
        const imageURL = await uploadFile(
          StorageFolder.ThrowableImage,
          imageFile,
        );
        const image: ItemImageConfig = {
          src: imageURL,
          pixelate: false,
          scale: 1,
          weight: 1,
        };

        const createItem: CreateItem = {
          config: {
            image,
            windup: {
              enabled: false,
              duration: 0,
            },
          },
          name: imageFile.name,
          impact_sounds: [],
          windup_sounds: [],
        };

        return createItem;
      }),
    );

    const createPromise = bulkCreateItem(createItems);

    toast.promise(createPromise, {
      loading: "Creating items...",
      success: "Created items",
      error: toastErrorMessage("Failed to create items"),
    });
  }
</script>

<Button
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  Bulk Create Throwables
</Button>

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
