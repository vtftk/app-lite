<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ScriptItem from "$lib/sections/scripts/ScriptItem.svelte";
  import type { UserScriptConfig } from "$shared/appData";
  import { Checkbox } from "bits-ui";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  let selected: string[] = $state([]);

  function onToggleSelected(item: UserScriptConfig) {
    if (selected.includes(item.id)) {
      selected = selected.filter((id) => id !== item.id);
    } else {
      selected = [...selected, item.id];
    }
  }

  const isAllSelected = $derived(selected.length === $appData.scripts.length);

  function onToggleAllSelected() {
    if (isAllSelected) {
      selected = [];
    } else {
      selected = $appData.scripts.map((item) => item.id);
    }
  }

  async function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected scripts?")) {
      return;
    }

    await $appDataMutation.mutateAsync({
      ...$appData,
      scripts: $appData.scripts.filter((item) => !selected.includes(item.id)),
    });

    selected = [];
  }
</script>

{#snippet actions()}
  <a class="btn" href="/scripts/create"> Create Script </a>
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
  title="Scripts"
  description="Create scripts that can handle events"
  {actions}
  {beforeContent}
>
  <div class="grid">
    {#each $appData.scripts as item}
      <ScriptItem
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
