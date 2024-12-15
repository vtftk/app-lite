<script lang="ts" generics="T extends { id: string }">
  import type { UpdateOrdering } from "$shared/dataV2";
  import type { Snippet } from "svelte";
  import {
    dndzone,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
    type DndEvent,
  } from "svelte-dnd-action";

  type Props = {
    items: T[];

    item: Snippet<[T]>;

    // Called when the ordering of the list is due for an update
    onUpdateOrder: (ordering: UpdateOrdering[]) => Promise<void>;
  };

  const { items: _items, item: renderItem, onUpdateOrder }: Props = $props();

  // Local state for list of items to allow reordering
  let items: T[] = $state([]);

  // Update the items when the props change
  $effect(() => {
    items = _items;
  });

  function handleDndConsider(e: CustomEvent<DndEvent<T>>) {
    items = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<DndEvent<T>>) {
    items = e.detail.items;
    onUpdateOrder(items.map((item, index) => ({ id: item.id, order: index })));
  }
</script>

<div
  class="grid"
  use:dndzone={{ items }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  {#each items as item (item.id)}
    <div class="item-wrapper">
      {@render renderItem(item)}

      {#if (item as any)[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
        <div class="custom-shadow-item"></div>
      {/if}
    </div>
  {/each}
</div>

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

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    width: 100%;
  }
</style>
