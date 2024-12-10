<script lang="ts">
  import { bulkDeleteEventMutation, createEventsQuery } from "$lib/api/vevents";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import EventItem from "$lib/sections/events/EventItem.svelte";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import type { EventId } from "$shared/dataV2";

  const eventsQuery = createEventsQuery();
  const bulkDeleteEvent = bulkDeleteEventMutation();

  // Readable access to the items from the underlying items query
  const events = $derived($eventsQuery.data ?? []);

  let selected: string[] = $state([]);

  function onToggleSelected(item: EventId) {
    if (selected.includes(item)) {
      selected = selected.filter((id) => id !== item);
    } else {
      selected = [...selected, item];
    }
  }

  function onToggleAllSelected() {
    if (events.length > 0 && selected.length === events.length) {
      selected = [];
    } else {
      selected = events.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected events?")) {
      return;
    }

    const deletePromise = $bulkDeleteEvent.mutateAsync({ eventIds: selected });

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
      checked={events.length > 0 && selected.length === events.length}
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
    {#each events as event}
      <EventItem
        config={event}
        selected={selected.includes(event.id)}
        onToggleSelected={() => onToggleSelected(event.id)}
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
