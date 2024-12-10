<script lang="ts">
  import {
    bulkDeleteScriptMutation,
    createScriptsQuery,
  } from "$lib/api/scripts";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ScriptItem from "$lib/sections/scripts/ScriptItem.svelte";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import type { ScriptId } from "$shared/dataV2";

  const scriptsQuery = createScriptsQuery();
  const bulkDeleteScripts = bulkDeleteScriptMutation();

  // Readable access to the items from the underlying items query
  const scripts = $derived($scriptsQuery.data ?? []);

  let selected: ScriptId[] = $state([]);

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
      error: "Failed to delete scripts",
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
    <Checkbox.Root
      checked={scripts.length > 0 && selected.length === scripts.length}
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
  title="Scripts"
  description="Create scripts that can handle events"
  {actions}
  {beforeContent}
>
  <div class="grid">
    {#each scripts as item}
      <ScriptItem
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
