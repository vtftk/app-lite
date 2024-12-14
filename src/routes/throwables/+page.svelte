<script lang="ts">
  import { flip } from "svelte/animate";
  import { getRuntimeAppData } from "$lib/api/runtimeAppData";
  import BulkThrowableImport from "$lib/components/throwable/BulkThrowableImport.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import BulkAddThrowableSounds from "$lib/sections/throwables/BulkAddThrowableSounds.svelte";
  import ThrowableItem from "$lib/sections/throwables/ThrowableItem.svelte";
  import { Checkbox } from "bits-ui";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import { toast } from "svelte-sonner";
  import {
    bulkAppendItemSoundsMutation,
    bulkDeleteItemsMutation,
    createItemsQuery,
    updateItemMutation,
    updateItemOrder,
    updateItemsMutation,
  } from "$lib/api/items";
  import type { Item, Sound } from "$shared/dataV2";
  import { toastErrorMessage } from "$lib/utils/error";
  import {
    dndzone,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
    type DndEvent,
  } from "svelte-dnd-action";

  const runtimeAppData = getRuntimeAppData();

  const itemsQuery = createItemsQuery();

  const bulkAppendItemSounds = bulkAppendItemSoundsMutation();
  const bulkDeleteItems = bulkDeleteItemsMutation();

  let items: Item[] = $state([]);

  // Readable access to the items from the underlying items query
  $effect(() => {
    items = $itemsQuery.data ?? [];
  });

  // Testing is only available when an overlay and vtube studio is connected
  const testingEnabled = $derived(
    $runtimeAppData.active_overlay_count > 0 &&
      $runtimeAppData.vtube_studio_connected
  );

  let selected: string[] = $state([]);

  function onToggleSelected(item: Item) {
    if (selected.includes(item.id)) {
      selected = selected.filter((id) => id !== item.id);
    } else {
      selected = [...selected, item.id];
    }
  }

  function onToggleAllSelected() {
    if (selected.length > 0 && selected.length === items.length) {
      selected = [];
    } else {
      selected = items.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected throwables?")) {
      return;
    }

    const deletePromise = $bulkDeleteItems.mutateAsync({
      itemIds: selected,
    });

    toast.promise(deletePromise, {
      loading: "Deleting items...",
      success: "Deleted items",
      error: "Failed to delete items",
    });

    selected = [];
  }

  function onBulkAddSounds(sounds: Sound[]) {
    if (
      !confirm(
        "Are you sure you want to add the selected impact sounds to the selected throwables?"
      )
    ) {
      return;
    }

    const impactSoundIds = sounds.map((sound) => sound.id);

    const addPromise = $bulkAppendItemSounds.mutateAsync({
      itemIds: selected,
      soundIds: impactSoundIds,
    });

    toast.promise(addPromise, {
      loading: "Adding impact sounds...",
      success: "Added impact sounds",
      error: "Failed to add impact sounds",
    });
  }

  function onTestThrow() {
    const throwPromise = testThrow(selected, 1);

    toast.promise(throwPromise, {
      loading: "Sending throw...",
      success: "Threw item",
      error: toastErrorMessage("Failed to throw item"),
    });
  }

  function onTestBarrage() {
    const throwPromise = testThrowBarrage(selected, 20, 2, 100);

    toast.promise(throwPromise, {
      loading: "Sending barrage...",
      success: "Threw barrage",
      error: toastErrorMessage("Failed to throw barrage"),
    });
  }

  function handleDndConsider(e: CustomEvent<DndEvent<Item>>) {
    items = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<DndEvent<Item>>) {
    items = e.detail.items;
    updateItemOrder(
      items.map((item, index) => ({ id: item.id, order: index }))
    );
  }
</script>

{#snippet actions()}
  <a class="btn" href="/throwables/create"> Create Throwable </a>
  <BulkThrowableImport />
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <Checkbox.Root
      checked={selected.length > 0 && selected.length === items.length}
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
        <button
          type="button"
          class="btn"
          onclick={onTestThrow}
          disabled={!testingEnabled}
        >
          <BallIcon /> Test
        </button>
        <button
          type="button"
          class="btn"
          onclick={onTestBarrage}
          disabled={!testingEnabled}
        >
          <BallsIcon /> Test Barrage
        </button>

        <BulkAddThrowableSounds onSubmit={onBulkAddSounds} />

        <button class="btn" onclick={onBulkDelete}>
          <DeleteIcon /> Delete
        </button>
      </div>
    {/if}
  </div>
{/snippet}

<PageLayoutList
  title="Throwables"
  description="Items that can be thrown. Configure them below"
  {actions}
  {beforeContent}
>
  <div
    class="grid"
    use:dndzone={{ items }}
    onconsider={handleDndConsider}
    onfinalize={handleDndFinalize}
  >
    {#each items as item (item.id)}
      <div class="item-wrapper">
        <ThrowableItem
          config={item}
          selected={selected.includes(item.id)}
          onToggleSelected={() => onToggleSelected(item)}
        />

        {#if (item as any)[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
          <div class="custom-shadow-item"></div>
        {/if}
      </div>
    {/each}
  </div>
</PageLayoutList>

<style>
  .item-wrapper {
    position: relative;
  }

  .custom-shadow-item {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    visibility: visible;
    border: 3px dashed #444;
    background: #212121;
    opacity: 0.5;
    margin: 0;
  }

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
