<script lang="ts">
  import {
    bulkDeleteScriptMutation,
    createScriptsQuery,
    updateScriptOrder,
  } from "$lib/api/scripts";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ScriptItem from "$lib/sections/scripts/ScriptItem.svelte";
  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import type { Script, ScriptId } from "$shared/dataV2";
  import {
    dndzone,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
    type DndEvent,
  } from "svelte-dnd-action";
  import { toastErrorMessage } from "$lib/utils/error";

  const scriptsQuery = createScriptsQuery();
  const bulkDeleteScripts = bulkDeleteScriptMutation();

  let scripts: Script[] = $state([]);

  // Readable access to the items from the underlying items query
  $effect(() => {
    scripts = $scriptsQuery.data ?? [];
  });

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
      error: toastErrorMessage("Failed to delete scripts"),
    });

    // Clear selection since all items are removed
    selected = [];
  }

  function handleDndConsider(e: CustomEvent<DndEvent<Script>>) {
    scripts = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<DndEvent<Script>>) {
    scripts = e.detail.items;
    updateScriptOrder(
      scripts.map((script, index) => ({ id: script.id, order: index }))
    );
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
  <div
    class="grid"
    use:dndzone={{ items: scripts }}
    onconsider={handleDndConsider}
    onfinalize={handleDndFinalize}
  >
    {#each scripts as item (item.id)}
      <div class="item-wrapper">
        <ScriptItem
          config={item}
          selected={selected.includes(item.id)}
          onToggleSelected={() => onToggleSelected(item.id)}
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
