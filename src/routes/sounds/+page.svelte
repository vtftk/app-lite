<script lang="ts">
  import type { Sound } from "$shared/dataV2";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import SoundItem from "$lib/sections/sounds/SoundItem.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import SearchInput from "$lib/components/form/SearchInput.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import { confirmDialog } from "$lib/components/GlobalConfirmDialog.svelte";
  import BulkSoundImport from "$lib/components/sounds/BulkSoundImport.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import {
    deleteSounds,
    updateSoundOrder,
    createSoundsQuery,
  } from "$lib/api/soundModel";

  const soundsQuery = createSoundsQuery();

  let search = $state("");
  let selected: string[] = $state([]);

  const sounds: Sound[] = $derived(
    filterItemsSearch($soundsQuery.data ?? [], search),
  );

  function filterItemsSearch(options: Sound[], search: string) {
    search = search.trim().toLowerCase();

    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  function onToggleSelected(item: Sound) {
    if (selected.includes(item.id)) {
      selected = selected.filter((id) => id !== item.id);
    } else {
      selected = [...selected, item.id];
    }
  }

  function onToggleAllSelected() {
    if (selected.length > 0 && selected.length === sounds.length) {
      selected = [];
    } else {
      selected = sounds.map((item) => item.id);
    }
  }

  async function onBulkDelete() {
    const confirm = await confirmDialog({
      title: "Confirm Delete",
      description: "Are you sure you want to delete the selected sounds?",
    });

    if (!confirm) {
      return;
    }

    const deletePromise = deleteSounds(selected);

    toast.promise(deletePromise, {
      loading: "Deleting sounds...",
      success: "Deleted sounds",
      error: toastErrorMessage("Failed to delete sounds"),
    });

    selected = [];
  }
</script>

{#snippet actions()}
  <PopoverButton content={createPopoverContent}>Create</PopoverButton>
{/snippet}

{#snippet createPopoverContent()}
  <LinkButton href="/sounds/create">Create Sound</LinkButton>
  <BulkSoundImport />
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <ControlledCheckbox
      checked={selected.length > 0 && selected.length === sounds.length}
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
      <Button onclick={onBulkDelete} disabled={selected.length < 1}>
        <DeleteIcon /> Delete
      </Button>
    </div>
  </div>
{/snippet}

<!-- Snippet for rendering items within the grid -->
{#snippet item(sound: Sound)}
  <SoundItem
    config={sound}
    selected={selected.includes(sound.id)}
    onToggleSelected={() => onToggleSelected(sound)}
  />
{/snippet}

<PageLayoutList
  title="Sounds"
  description="Create sounds that can be used for events or use as impact sounds"
  {actions}
  {beforeContent}
>
  <OrderableGrid
    items={sounds}
    {item}
    onUpdateOrder={updateSoundOrder}
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

  .selection__gap {
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
