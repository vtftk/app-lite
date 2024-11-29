<script lang="ts">
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import BulkThrowableImport from "$lib/components/throwable/BulkThrowableImport.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import BulkAddThrowableSounds from "$lib/sections/throwables/BulkAddThrowableSounds.svelte";
  import ThrowableItem from "$lib/sections/throwables/ThrowableItem.svelte";
  import type { ItemConfig, SoundConfig } from "$shared/appData";
  import { Checkbox } from "bits-ui";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  let selected: string[] = $state([]);

  function onToggleSelected(item: ItemConfig) {
    if (selected.includes(item.id)) {
      selected = selected.filter((id) => id !== item.id);
    } else {
      selected = [...selected, item.id];
    }
  }

  const isAllSelected = $derived(selected.length === $appData.items.length);

  function onToggleAllSelected() {
    if (isAllSelected) {
      selected = [];
    } else {
      selected = $appData.items.map((item) => item.id);
    }
  }

  async function onBulkDelete() {
    if (!confirm("Are you sure you want to delete the selected throwables?")) {
      return;
    }

    await $appDataMutation.mutateAsync({
      ...$appData,
      items: $appData.items.filter((item) => !selected.includes(item.id)),
    });

    selected = [];
  }

  async function onBulkAddSounds(sounds: SoundConfig[]) {
    if (
      !confirm(
        "Are you sure you want to add the selected impact sounds to the selected throwables?"
      )
    ) {
      return;
    }

    const impactSoundIds = sounds.map((sound) => sound.id);

    await $appDataMutation.mutateAsync({
      ...$appData,
      items: $appData.items.map((item) => {
        if (selected.includes(item.id)) {
          return {
            ...item,
            impact_sounds_ids: [
              ...item.impact_sounds_ids,
              // Add new sounds filtering out ones that are already present
              ...impactSoundIds.filter(
                (id) => !item.impact_sounds_ids.includes(id)
              ),
            ],
          };
        }

        return item;
      }),
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

        <BulkAddThrowableSounds
          sounds={$appData.sounds}
          onSubmit={onBulkAddSounds}
        />
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
    {#each $appData.items as item}
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
    grid-template-columns: 1fr;
    gap: 0.5rem;
    width: 100%;
  }
</style>
