<script lang="ts">
  import type { Sound } from "$lib/api/types";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import { Checkbox } from "bits-ui";
  import SoundPlayButton from "$lib/components/sounds/SoundPlayButton.svelte";
  import getBackendURL from "$lib/utils/url";
  import { deleteSoundMutation } from "$lib/api/sounds";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";

  type Props = {
    config: Sound;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const deleteSound = deleteSoundMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this sound item?")) {
      return;
    }

    const deletePromise = $deleteSound.mutateAsync(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting sound...",
      success: "Deleted sound",
      error: toastErrorMessage("Failed to delete sound"),
    });
  }
</script>

<div class="sound">
  <Checkbox.Root checked={selected} onCheckedChange={onToggleSelected}>
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

  <a class="sound__name" href="/sounds/{config.id}">{config.name}</a>

  <div class="sound__actions">
    <SoundPlayButton src={getBackendURL(config.src)} />
    <a class="sound-button" href="/sounds/{config.id}">
      <SettingsIcon />
    </a>
    <button class="sound-button" onclick={onDelete}> <DeleteIcon /> </button>
  </div>
</div>

<style>
  .sound {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    gap: 1rem;

    padding: 0.5rem;
    align-items: center;
  }

  .sound__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .sound__name {
    flex: 1;
    color: #fff;
    font-weight: bold;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .sound-button {
    padding: 0.5rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .sound-button:hover {
    background-color: #444;
  }
</style>
