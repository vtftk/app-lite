<script lang="ts">
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import type { Script, ScriptId } from "$shared/dataV2";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import ScriptItem from "$lib/sections/scripts/ScriptItem.svelte";
  import SearchInput from "$lib/components/form/SearchInput.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import {
    updateScriptOrder,
    createScriptsQuery,
    bulkDeleteScriptMutation,
  } from "$lib/api/scripts";

  const scriptsQuery = createScriptsQuery();
  const bulkDeleteScripts = bulkDeleteScriptMutation();

  let search = $state("");
  let selected: string[] = $state([]);

  const scripts = $derived(filterItemsSearch($scriptsQuery.data ?? [], search));

  function filterItemsSearch(options: Script[], search: string) {
    search = search.trim().toLowerCase();

    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  function onToggleSelected(item: ScriptId) {
    if (selected.includes(item)) {
      selected = selected.filter((id) => id !== item);
    } else {
      selected = [...selected, item];
    }
  }

  function onToggleAllSelected() {
    if (scripts.length > 0 && selected.length === scripts.length) {
      selected = [];
    } else {
      selected = scripts.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected scripts?")) {
      return;
    }

    const deletePromise = $bulkDeleteScripts.mutateAsync({
      scriptIds: selected,
    });

    toast.promise(deletePromise, {
      loading: "Deleting scripts...",
      success: "Deleted scripts",
      error: toastErrorMessage("Failed to delete scripts"),
    });

    // Clear selection since all items are removed
    selected = [];
  }
</script>

{#snippet actions()}
  <a class="btn" href="/scripts/create"> Create Script </a>
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <ControlledCheckbox
      checked={selected.length > 0 && selected.length === scripts.length}
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
{#snippet item(item: Script)}
  <ScriptItem
    config={item}
    selected={selected.includes(item.id)}
    onToggleSelected={() => onToggleSelected(item.id)}
  />
{/snippet}

<PageLayoutList
  title="Scripts"
  description="Create scripts that can handle events"
  {actions}
  {beforeContent}
>
  <OrderableGrid
    items={scripts}
    {item}
    onUpdateOrder={updateScriptOrder}
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
