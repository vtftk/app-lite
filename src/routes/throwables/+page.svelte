<script lang="ts">
  import type { Item, Sound, ItemCollectionWithItems } from "$shared/dataV2";

  import { Tabs } from "bits-ui";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import Button from "$lib/components/input/Button.svelte";
  import { getRuntimeAppData } from "$lib/api/runtimeAppData";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import OrderableList from "$lib/components/OrderableList.svelte";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import SearchInput from "$lib/components/form/SearchInput.svelte";
  import SoundPicker from "$lib/components/sounds/SoundPicker.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import ThrowableItem from "$lib/sections/throwables/ThrowableItem.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import PopoverCloseButton from "$lib/components/popover/PopoverCloseButton.svelte";
  import BulkThrowableImport from "$lib/components/throwable/BulkThrowableImport.svelte";
  import ThrowableCollectionItem from "$lib/sections/throwables/ThrowableCollectionItem.svelte";
  import {
    updateItemOrder,
    createItemsQuery,
    bulkDeleteItemsMutation,
    bulkAppendItemSoundsMutation,
  } from "$lib/api/items";
  import {
    mergeItemCollectionItems,
    createItemCollectionsQuery,
    updateItemCollectionOrderings,
  } from "$lib/api/itemCollections";
  import CreateItemCollectionForm from "$lib/sections/throwables/CreateItemCollectionForm.svelte";

  const runtimeAppData = getRuntimeAppData();

  const itemsQuery = createItemsQuery();
  const itemCollectionsQuery = createItemCollectionsQuery();

  const bulkAppendItemSounds = bulkAppendItemSoundsMutation();
  const bulkDeleteItems = bulkDeleteItemsMutation();

  let search = $state("");
  let selected: string[] = $state([]);

  const items = $derived(filterItemsSearch($itemsQuery.data ?? [], search));
  const itemCollections = $derived($itemCollectionsQuery.data ?? []);

  const itemCollectionsWithItems = $derived(
    itemCollections.map((collection) =>
      mergeItemCollectionItems(collection, items),
    ),
  );

  function filterItemsSearch(options: Item[], search: string) {
    search = search.trim().toLowerCase();

    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  // Testing is only available when an overlay and vtube studio is connected
  const testingEnabled = $derived(
    $runtimeAppData.active_overlay_count > 0 &&
      $runtimeAppData.vtube_studio_connected,
  );

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
      error: toastErrorMessage("Failed to delete items"),
    });

    selected = [];
  }

  function onBulkAddSounds(sounds: Sound[]) {
    if (
      !confirm(
        "Are you sure you want to add the selected impact sounds to the selected throwables?",
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
      error: toastErrorMessage("Failed to add impact sounds"),
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
</script>

<!-- Actions in the titlebar -->
{#snippet actions()}
  <PopoverButton content={createPopoverContent}>Create</PopoverButton>
{/snippet}

<!-- Content for the "Test" button popover -->
{#snippet createPopoverContent()}
  <a class="btn" href="/throwables/create"> Create Throwable </a>
  <BulkThrowableImport />
  <CreateItemCollectionForm />
{/snippet}

<!-- Content for the "Test" button popover -->
{#snippet testPopoverContent()}
  <PopoverCloseButton onclick={onTestThrow}>
    <BallIcon /> Test One
  </PopoverCloseButton>

  <PopoverCloseButton onclick={onTestBarrage}>
    <BallsIcon /> Test Barrage
  </PopoverCloseButton>
{/snippet}

{#snippet bulkAddSoundButton()}
  <SettingsIcon />Add Impact Sounds
{/snippet}

<!-- Section before the content -->
{#snippet beforeContent()}
  <div class="selection">
    <ControlledCheckbox
      checked={selected.length > 0 && selected.length === items.length}
      onCheckedChange={onToggleAllSelected}
    />

    <div class="search-wrapper">
      <SearchInput bind:value={search} placeholder="Search..." />
    </div>

    {#if selected.length > 0}
      <div class="selection__count">
        {selected.length} Selected
      </div>
    {/if}

    <div class="selection__gap"></div>

    <div class="selection__actions">
      <PopoverButton
        content={testPopoverContent}
        disabled={!testingEnabled || selected.length < 1}
      >
        <BallIcon /> Test
      </PopoverButton>

      <SoundPicker
        disabled={selected.length < 1}
        buttonContent={bulkAddSoundButton}
        description="Choose which impact sounds you'd like to add the the selected throwables."
        selected={[]}
        onChangeSelected={onBulkAddSounds}
      />

      <Button onclick={onBulkDelete} disabled={selected.length < 1}>
        <DeleteIcon /> Delete
      </Button>
    </div>
  </div>
{/snippet}

<!-- Snippet for rendering items within the grid -->
{#snippet item(item: Item)}
  <ThrowableItem
    config={item}
    selected={selected.includes(item.id)}
    onToggleSelected={() => onToggleSelected(item)}
  />
{/snippet}

{#snippet itemCollection(item: ItemCollectionWithItems)}
  <ThrowableCollectionItem collection={item} />
{/snippet}

<PageLayoutList
  title="Throwables"
  description="Items that can be thrown. Configure them below"
  {actions}
  {beforeContent}
>
  <div class="content">
    <Tabs.Root>
      <Tabs.List>
        <Tabs.Trigger value="all">All</Tabs.Trigger>
        <Tabs.Trigger value="collections">Collections</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="all">
        <OrderableGrid
          {items}
          {item}
          onUpdateOrder={updateItemOrder}
          disableOrdering={search.length > 0}
        />
      </Tabs.Content>
      <Tabs.Content value="collections">
        <OrderableList
          items={itemCollectionsWithItems}
          item={itemCollection}
          onUpdateOrder={updateItemCollectionOrderings}
          disableOrdering={search.length > 0}
        />
      </Tabs.Content>
    </Tabs.Root>
  </div>
</PageLayoutList>

<style>
  .content {
    position: relative;
    flex: auto;
    overflow: hidden;
    height: 100%;
  }

  .content :global([data-tabs-root]) {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .content :global([data-tabs-content]) {
    position: relative;
    flex: auto;
    overflow: auto;
    flex-flow: column;
    border: 1px solid #333;
  }

  .selection {
    display: flex;
    align-items: center;
    gap: 1rem;
    height: 3rem;
    flex-shrink: 0;
  }

  .selection__gap {
    flex: auto;
  }

  .selection__actions {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .search-wrapper {
    display: flex;
    flex: auto;
    flex-shrink: 1;
    flex-grow: 0;
    max-width: 20rem;
  }
</style>
