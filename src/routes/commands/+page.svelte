<script lang="ts">
  import {
    bulkDeleteCommandMutation,
    createCommandsQuery,
  } from "$lib/api/commands";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import CommandItem from "$lib/sections/commands/CommandItem.svelte";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import type { CommandId } from "$shared/dataV2";
  import { toastErrorMessage } from "$lib/utils/error";

  const commandsQuery = createCommandsQuery();
  const bulkDeleteCommand = bulkDeleteCommandMutation();

  // Readable access to the items from the underlying items query
  const commands = $derived($commandsQuery.data ?? []);

  let selected: string[] = $state([]);

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
    <Checkbox.Root
      checked={commands.length > 0 && selected.length === commands.length}
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
  title="Commands"
  description="Create custom commands"
  {actions}
  {beforeContent}
>
  <div class="grid">
    {#each commands as item}
      <CommandItem
        config={item}
        selected={selected.includes(item.id)}
        onToggleSelected={() => onToggleSelected(item.id)}
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
