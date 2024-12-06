<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import type { EventConfig } from "$lib/api/types";
  import { Checkbox } from "bits-ui";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    config: EventConfig;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this event item?")) {
      return;
    }

    await $appDataMutation.mutateAsync({
      ...$appData,
      events: $appData.events.filter((event) => event.id !== config.id),
    });
  }
</script>

<div class="event">
  <Checkbox.Root checked={selected} onCheckedChange={onToggleSelected}>
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

  <p class="event__name">{config.name}</p>

  <div class="event__actions">
    <a class="throw-button" href="/events/{config.id}">
      <SettingsIcon />
    </a>
    <button class="throw-button" onclick={onDelete}> <DeleteIcon /> </button>
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

  .event__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .event__name {
    color: #fff;
    font-weight: bold;
    display: flex;
    flex: auto;
    align-items: center;
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
