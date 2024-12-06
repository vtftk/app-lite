<script lang="ts">
  import {
    createAppDateMutation,
    createDeleteEventsMutation,
    getAppData,
  } from "$lib/api/runtimeAppData";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import EventItem from "$lib/sections/events/EventItem.svelte";
  import type { EventConfig } from "$shared/appData";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const deleteEvents = createDeleteEventsMutation(appData, appDataMutation);

  let selected: string[] = $state([]);

  const isAllSelected = $derived(
    selected.length > 0 && selected.length === $appData.events.length
  );

  function onToggleSelected(item: EventConfig) {
    if (selected.includes(item.id)) {
      selected = selected.filter((id) => id !== item.id);
    } else {
      selected = [...selected, item.id];
    }
  }

  function onToggleAllSelected() {
    if (isAllSelected) {
      selected = [];
    } else {
      selected = $appData.events.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected events?")) {
      return;
    }

    const deletePromise = $deleteEvents(selected);

    toast.promise(deletePromise, {
      loading: "Deleting events...",
      success: "Deleted events",
      error: "Failed to delete events",
    });

    // Clear selection since all items are removed
    selected = [];
  }
</script>

{#snippet actions()}
  <a class="btn" href="/events/create"> Create Event </a>
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <Checkbox.Root
      checked={isAllSelected}
      onCheckedChange={onToggleAllSelected}
    >
      <Checkbox.Indicator let:isChecked>
        {#if isChecked}
          <span>&#10003;</span>
        {/if}
      </Checkbox.Indicator>
    </Checkbox.Root>

    {#if selected.length > 0}
      <div class="selection__count">
        {selected.length} Selected
      </div>

      <div class="selection__actions">
        <button class="btn" onclick={onBulkDelete}><DeleteIcon /> Delete</button
        >
      </div>
    {/if}
  </div>
{/snippet}

<PageLayoutList
  title="Events"
  description="Setup specific triggers for events, such as throwing when a specific redeem is redeemed"
  {actions}
  {beforeContent}
>
  <div class="grid">
    {#each $appData.events as event}
      <EventItem
        config={event}
        selected={selected.includes(event.id)}
        onToggleSelected={() => onToggleSelected(event)}
      />
    {/each}
  </div>
</PageLayoutList>

<style>
  .selection {
    display: flex;
    align-items: center;
    gap: 1rem;
    height: 3rem;
    flex-shrink: 0;
  }

  .selection__count {
    flex: auto;
  }

  .selection__actions {
    display: flex;
    gap: 1rem;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    width: 100%;
  }
</style>
