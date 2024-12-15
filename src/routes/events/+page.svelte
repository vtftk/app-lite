<script lang="ts">
  import type { VEvent, EventId } from "$shared/dataV2";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import EventItem from "$lib/sections/events/EventItem.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import SearchInput from "$lib/components/form/SearchInput.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import {
    updateEventOrder,
    createEventsQuery,
    bulkDeleteEventMutation,
  } from "$lib/api/vevents";

  const eventsQuery = createEventsQuery();
  const bulkDeleteEvent = bulkDeleteEventMutation();

  let search = $state("");
  let selected: string[] = $state([]);

  const events = $derived(filterItemsSearch($eventsQuery.data ?? [], search));

  function filterItemsSearch(options: VEvent[], search: string) {
    search = search.trim().toLowerCase();

    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

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
      error: toastErrorMessage("Failed to delete events"),
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
    <ControlledCheckbox
      checked={selected.length > 0 && selected.length === events.length}
      onCheckedChange={onToggleAllSelected}
    />

    <div class="search-wrapper">
      <SearchInput bind:value={search} placeholder="Search..." />
    </div>

    {#if selected.length > 0}
      <div class="selection__count">
        {selected.length} Selected
      </div>

      <div class="selection__actions">
        <Button onclick={onBulkDelete}><DeleteIcon /> Delete</Button>
      </div>
    {/if}
  </div>
{/snippet}

<!-- Snippet for rendering items within the grid -->
{#snippet item(event: VEvent)}
  <EventItem
    config={event}
    selected={selected.includes(event.id)}
    onToggleSelected={() => onToggleSelected(event.id)}
  />
{/snippet}

<PageLayoutList
  title="Events"
  description="Setup specific triggers for events, such as throwing when a specific redeem is redeemed"
  {actions}
  {beforeContent}
>
  <OrderableGrid
    items={events}
    {item}
    onUpdateOrder={updateEventOrder}
    disableOrdering={search.length > 0}
  />
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

  .search-wrapper {
    display: flex;
    flex: auto;
    flex-shrink: 1;
    flex-grow: 0;
    max-width: 20rem;
  }
</style>
