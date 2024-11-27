<script lang="ts">
  import type { ItemConfig, ThrowableConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import { getAppData } from "$lib/api/runtimeAppData";
  import { app } from "@tauri-apps/api";

  type Props = {
    config: ItemConfig;
  };

  const appData = getAppData();
  const { config }: Props = $props();

  async function testThrow() {
    const impact_sounds = $appData.sounds.filter((sound) =>
      config.impact_sounds_ids.includes(sound.id)
    );

    const throwable: ThrowableConfig = {
      items: [config],
      impact_sounds,
    };

    await invoke("test_throw", {
      config: throwable,
      amount: 1,
    });
  }

  async function testThrowMany() {
    const impact_sounds = $appData.sounds.filter((sound) =>
      config.impact_sounds_ids.includes(sound.id)
    );

    const throwable: ThrowableConfig = {
      items: [config],
      impact_sounds,
    };

    await invoke("test_throw_barrage", {
      config: throwable,
      amount: 50,
      amountPerThrow: 2,
      frequency: 100,
    });
  }
</script>

<div class="throwable">
  <div class="throwable__content">
    <div class="throwable__image-wrapper">
      <img class="throwable__image" src={config.image.src} alt="Throwable" />
    </div>
    <p class="throwable__name">{config.name}</p>
  </div>

  <div class="throwable__actions">
    <a class="throw-button" href="/throwables/{config.id}">
      <SettingsIcon />
    </a>
    <button class="throw-button" onclick={() => {}}> <DeleteIcon /> </button>

    <button class="throw-button" onclick={testThrow}><BallIcon /></button>
    <button class="throw-button" onclick={testThrowMany}><BallsIcon /></button>
  </div>
</div>

<style>
  .throwable {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    gap: 0.5rem;

    padding: 0.5rem;
  }

  .throwable__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .throwable__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .throwable__image {
    width: 2.5rem;
    height: 2.5rem;
    object-fit: contain;
    background-color: #333;
    border-radius: 2rem;
  }

  .throwable__name {
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
