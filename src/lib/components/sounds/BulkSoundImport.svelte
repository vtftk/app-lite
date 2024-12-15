<script lang="ts">
  import type { CreateSound } from "$shared/dataV2";

  import { toast } from "svelte-sonner";
  import { FileType } from "$lib/api/types";
  import { uploadFile } from "$lib/api/data";
  import { toastErrorMessage } from "$lib/utils/error";
  import { bulkCreateSoundMutation } from "$lib/api/sounds";

  const bulkCreateSound = bulkCreateSoundMutation();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeSound() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const sounds = Array.from(files);

    const createSounds = await Promise.all(
      sounds.map(async (sound) => {
        const soundURL = await uploadFile(FileType.Sound, sound);
        const createSound: CreateSound = {
          src: soundURL,
          volume: 1,
          name: sound.name,
        };

        return createSound;
      }),
    );

    const createPromise = $bulkCreateSound.mutateAsync(createSounds);

    toast.promise(createPromise, {
      loading: "Creating sounds...",
      success: "Created sounds",
      error: toastErrorMessage("Failed to create sounds"),
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
