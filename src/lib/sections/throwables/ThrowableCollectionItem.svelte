<script lang="ts">
  import type { ItemWithOrder, ItemCollectionWithItems } from "$lib/api/types";

  import { toast } from "svelte-sonner";
  import { slide } from "svelte/transition";
  import { deleteItemMutation } from "$lib/api/items";
  import { toastErrorMessage } from "$lib/utils/error";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import SolarMenuDotsBold from "~icons/solar/menu-dots-bold";
  import OrderableGrid from "$lib/components/OrderableGrid.svelte";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import {
    deleteItemCollection,
    updateItemCollectionItemOrderings,
  } from "$lib/api/itemCollections";

  import ThrowableItem from "./ThrowableItem.svelte";
  import CreateItemCollectionForm from "./CreateItemCollectionForm.svelte";

  type Props = {
    collection: ItemCollectionWithItems;
  };

  const { collection }: Props = $props();

  const deleteItem = deleteItemMutation();

  let expanded = $state(false);

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this item collection?")) {
      return;
    }

    const deletePromise = deleteItemCollection(collection.id);

    toast.promise(deletePromise, {
      loading: "Deleting item collection...",
      success: "Deleted item collection",
      error: toastErrorMessage("Failed to delete item collection"),
    });
  }
</script>

{#snippet popoverContent()}
  <Button onclick={onDelete} disabled={$deleteItem.isPending}>
    <DeleteIcon /> Delete
  </Button>
  <CreateItemCollectionForm existing={collection} />
{/snippet}

<!-- Snippet for rendering items within the grid -->
{#snippet item(item: ItemWithOrder)}
  <ThrowableItem config={item} />
{/snippet}

<div class="item">
  <div class="item__head" class:item__head--expanded={expanded}>
    <div class="item__text">
      <p class="item__name">{collection.name}</p>
    </div>

    <PopoverButton
      content={popoverContent}
      contentProps={{ align: "start", side: "left" }}
    >
      <SolarMenuDotsBold />
    </PopoverButton>

    <Button onclick={() => (expanded = !expanded)}>
      {#if expanded}
        <SolarAltArrowUpBold />
      {:else}
        <SolarAltArrowDownBold />
      {/if}
    </Button>
  </div>

  {#if expanded}
    <div class="item__content" transition:slide={{ axis: "y", duration: 100 }}>
      <OrderableGrid
        items={collection.items}
        {item}
        onUpdateOrder={(update) =>
          updateItemCollectionItemOrderings(collection.id, update)}
      />
    </div>
  {/if}
</div>

<style>
  .item {
    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    border-radius: 5px;

    display: flex;
    flex-direction: column;

    padding: 0.5rem;
    overflow: hidden;
  }

  .item__head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;

    padding: 0.5rem;
    overflow: hidden;
    transition: padding-bottom 0.25s ease;
  }

  .item__head--expanded {
    padding-bottom: 1rem;
  }

  .item__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .item__text {
    display: flex;
    flex: auto;
    align-items: center;
    overflow: hidden;
  }

  .item__name {
    flex: 1;
    color: #fff;
    font-weight: bold;

    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;

    text-decoration: none;
  }

  .item__name:hover {
    text-decoration: underline;
  }
</style>
