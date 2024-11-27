<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import type { SoundConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import PlayIcon from "~icons/solar/play-bold";
  import { Checkbox } from "bits-ui";

  type Props = {
    config: SoundConfig;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  async function testSound() {
    await invoke("test_sound", {
      config,
    });
  }

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
  <Checkbox.Root checked={selected} onCheckedChange={onToggleSelected}>
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

  <a class="sound__name" href="/sounds/{config.id}">{config.name}</a>

  <div class="sound__actions">
    <button class="sound-button" onclick={testSound}><PlayIcon /></button>
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
