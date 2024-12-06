<script lang="ts">
  import {
    createAppDateMutation,
    createDeleteCommandsMutation,
    getAppData,
  } from "$lib/api/runtimeAppData";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import CommandItem from "$lib/sections/commands/CommandItem.svelte";
  import type { CommandConfig } from "$shared/appData";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const deleteCommands = createDeleteCommandsMutation(appData, appDataMutation);

  let selected: string[] = $state([]);
  const isAllSelected = $derived(selected.length === $appData.commands.length);

  function onToggleSelected(item: CommandConfig) {
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
      selected = $appData.commands.map((item) => item.id);
    }
  }

  function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected commands?")) {
      return;
    }

    const deletePromise = $deleteCommands({
      commandIds: selected,
    });

    toast.promise(deletePromise, {
      loading: "Deleting commands...",
      success: "Deleted commands",
      error: "Failed to delete commands",
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
  title="Commands"
  description="Create custom commands"
  {actions}
  {beforeContent}
>
  <div class="grid">
    {#each $appData.commands as item}
      <CommandItem
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
