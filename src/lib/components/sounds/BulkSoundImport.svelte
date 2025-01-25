<script lang="ts">
  import type { CreateSound } from "$shared/dataV2";

  import { toast } from "svelte-sonner";
  import { uploadFile } from "$lib/api/data";
  import { StorageFolder } from "$lib/api/types";
  import { createSounds } from "$lib/api/soundModel";
  import { toastErrorMessage } from "$lib/utils/error";

  import Button from "../input/Button.svelte";

  type Props = {
    label?: string;
  };

  const { label = "Bulk Create Sounds" }: Props = $props();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeSound() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const sounds = Array.from(files);

    const creates = await Promise.all(
      sounds.map(async (sound) => {
        const soundURL = await uploadFile(StorageFolder.Sound, sound);
        const createSound: CreateSound = {
          src: soundURL,
          volume: 1,
          name: sound.name,
        };

        return createSound;
      }),
    );

    const createPromise = createSounds(creates);

    toast.promise(createPromise, {
      loading: "Creating sounds...",
      success: "Created sounds",
      error: toastErrorMessage("Failed to create sounds"),
    });
  }
</script>

<Button
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  {label}
</Button>

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
