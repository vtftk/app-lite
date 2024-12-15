<script lang="ts">
  import {
    bulkDeleteCommandMutation,
    createCommandsQuery,
    updateCommandOrder,
  } from "$lib/api/commands";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import CommandItem from "$lib/sections/commands/CommandItem.svelte";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import type { Command, CommandId } from "$shared/dataV2";
  import { toastErrorMessage } from "$lib/utils/error";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import SearchInput from "$lib/components/form/SearchInput.svelte";

  const commandsQuery = createCommandsQuery();
  const bulkDeleteCommand = bulkDeleteCommandMutation();

  let search = $state("");
  let selected: string[] = $state([]);

  const commands = $derived(
    filterItemsSearch($commandsQuery.data ?? [], search)
  );

  function filterItemsSearch(options: Command[], search: string) {
    search = search.trim().toLowerCase();

    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  function onToggleSelected(item: CommandId) {
    if (selected.includes(item)) {
      selected = selected.filter((id) => id !== item);
    } else {
      selected = [...selected, item];
    }
  }

  function onToggleAllSelected() {
    if (commands.length > 0 && selected.length === commands.length) {
      selected = [];
    } else {
      selected = commands.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected commands?")) {
      return;
    }

    const deletePromise = $bulkDeleteCommand.mutateAsync({
      commandIds: selected,
    });

    toast.promise(deletePromise, {
      loading: "Deleting commands...",
      success: "Deleted commands",
      error: toastErrorMessage("Failed to delete commands"),
    });

    selected = [];
  }
</script>

{#snippet actions()}
  <a class="btn" href="/commands/create"> Create Command </a>
{/snippet}

{#snippet beforeContent()}
  <div class="selection">
    <ControlledCheckbox
      checked={selected.length > 0 && selected.length === commands.length}
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
{#snippet item(item: Command)}
  <CommandItem
    config={item}
    selected={selected.includes(item.id)}
    onToggleSelected={() => onToggleSelected(item.id)}
  />
{/snippet}

<PageLayoutList
  title="Commands"
  description="Create custom commands"
  {actions}
  {beforeContent}
>
  <OrderableGrid
    items={commands}
    {item}
    onUpdateOrder={updateCommandOrder}
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
