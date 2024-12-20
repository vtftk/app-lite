<script lang="ts">
  import type { CreateItem } from "$shared/dataV2";
  import type { ThrowableImageConfig } from "$shared/appData";

  import { toast } from "svelte-sonner";
  import { FileType } from "$lib/api/types";
  import { uploadFile } from "$lib/api/data";
  import { bulkCreateItem } from "$lib/api/items";
  import { toastErrorMessage } from "$lib/utils/error";

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeImage() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const images = Array.from(files);

    const createItems = await Promise.all(
      images.map(async (image) => {
        const imageURL = await uploadFile(FileType.ThrowableImage, image);
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
