<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import type { SoundConfig } from "$lib/api/types";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    config: SoundConfig;
  };

  const { config }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this sound item?")) {
      return;
    }

    await $appDataMutation.mutateAsync({
      ...$appData,
      sounds: $appData.sounds.filter((sound) => sound.id !== config.id),
    });
  }
</script>

<div class="sound">
  <div class="sound__content">
    <p class="sound__name">{config.name}</p>
  </div>

  <div class="sound__actions">
    <a class="throw-button" href="/sounds/{config.id}">
      <SettingsIcon />
    </a>
    <button class="throw-button" onclick={onDelete}> <DeleteIcon /> </button>
  </div>
</div>

<style>
  .sound {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    gap: 0.5rem;

    padding: 0.5rem;
  }

  .sound__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .sound__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .sound__name {
    color: #fff;
    font-weight: bold;
  }

  .throw-button {
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

  .throw-button:hover {
    background-color: #444;
  }
</style>
