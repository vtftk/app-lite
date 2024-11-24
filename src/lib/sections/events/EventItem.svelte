<script lang="ts">
  import type { EventConfig } from "$lib/api/types";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    config: EventConfig;
  };

  const { config }: Props = $props();
</script>

<div class="event">
  <div class="event__content">
    <p class="event__name">{JSON.stringify(config)}</p>
  </div>

  <div class="event__actions">
    <a class="throw-button" href="/events/{config.id}">
      <SettingsIcon />
    </a>
    <button class="throw-button" onclick={() => {}}> <DeleteIcon /> </button>
  </div>
</div>

<style>
  .event {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    gap: 0.5rem;

    padding: 0.5rem;
  }

  .event__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .event__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .event__name {
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
