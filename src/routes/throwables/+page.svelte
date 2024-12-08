<script lang="ts">
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import BulkThrowableImport from "$lib/components/throwable/BulkThrowableImport.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import BulkAddThrowableSounds from "$lib/sections/throwables/BulkAddThrowableSounds.svelte";
  import ThrowableItem from "$lib/sections/throwables/ThrowableItem.svelte";
  import type { SoundConfig } from "$shared/appData";
  import { Checkbox } from "bits-ui";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import { toast } from "svelte-sonner";
  import { derived as derivedStore } from "svelte/store";
  import {
    bulkAppendItemSoundsMutation,
    bulkDeleteItemsMutation,
    createItemsQuery,
  } from "$lib/api/items";
  import type { Item } from "$shared/dataV2";

  const runtimeAppData = getRuntimeAppData();

  const appData = getAppData();

  const itemsQuery = createItemsQuery();

  const bulkAppendItemSounds = bulkAppendItemSoundsMutation();
  const bulkDeleteItems = bulkDeleteItemsMutation();

  // Readable access to the items from the underlying items query
  const items = derivedStore(
    itemsQuery,
    ($itemsQuery) => $itemsQuery.data ?? []
  );

  // Testing is only available when an overlay and vtube studio is connected
  const testingEnabled = derivedStore(
    runtimeAppData,
    ($runtimeAppData) =>
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
    if (selected.length > 0 && selected.length === $items.length) {
      selected = [];
    } else {
      selected = $items.map((item) => item.id);
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

  function onBulkAddSounds(sounds: SoundConfig[]) {
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
      error: "Failed to throw item",
    });
  }

  function onTestBarrage() {
    const throwPromise = testThrowBarrage(selected, 50, 2, 100);

    toast.promise(throwPromise, {
      loading: "Sending barrage...",
      success: "Threw barrage",
      error: "Failed to throw barrage",
    });
  }
</script>

{#snippet actions()}
  <a class="btn" href="/throwables/create"> Create Throwable </a>
  <BulkThrowableImport />
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <Checkbox.Root
      checked={selected.length > 0 && selected.length === $items.length}
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
          disabled={!$testingEnabled}
        >
          <BallIcon /> Test
        </button>
        <button
          type="button"
          class="btn"
          onclick={onTestBarrage}
          disabled={!$testingEnabled}
        >
          <BallsIcon /> Test Barrage
        </button>

        <BulkAddThrowableSounds
          sounds={$appData.sounds}
          onSubmit={onBulkAddSounds}
        />

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
  <div class="grid">
    {#each $items as item}
      <ThrowableItem
        config={item}
        selected={selected.includes(item.id)}
        onToggleSelected={() => onToggleSelected(item)}
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
