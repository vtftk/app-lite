<script lang="ts">
  import type { VEvent } from "$lib/api/types";
  import { deleteEventMutation } from "$lib/api/vevents";
  import { toastErrorMessage } from "$lib/utils/error";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    config: VEvent;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const deleteEvent = deleteEventMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this event item?")) {
      return;
    }

    const deletePromise = $deleteEvent.mutateAsync(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting event...",
      success: "Deleted event",
      error: toastErrorMessage("Failed to delete event"),
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

  <a href="/events/{config.id}" class="event__name">{config.name}</a>

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
    align-items: center;
    gap: 1rem;

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
