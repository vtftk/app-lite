<script lang="ts">
  import type { Item } from "$lib/api/types";

  import { Checkbox } from "bits-ui";
  import { toast } from "svelte-sonner";
  import getBackendURL from "$lib/utils/url";
  import { deleteItemMutation } from "$lib/api/items";
  import { toastErrorMessage } from "$lib/utils/error";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    config: Item;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const deleteItem = deleteItemMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this item?")) {
      return;
    }

    const deletePromise = $deleteItem.mutateAsync(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting item...",
      success: "Deleted item",
      error: toastErrorMessage("Failed to delete item"),
    });
  }
</script>

<div class="throwable">
  <Checkbox.Root checked={selected} onCheckedChange={onToggleSelected}>
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

  <div class="throwable__content">
    <div class="throwable__image-wrapper">
      <img
        class="throwable__image"
        src={getBackendURL(config.image.src)}
        alt="Throwable"
      />
    </div>
  </div>
  <a href="/throwables/{config.id}" class="throwable__name">{config.name}</a>

  <div class="throwable__actions">
    <a class="throw-button" href="/throwables/{config.id}">
      <SettingsIcon />
    </a>
    <button
      class="throw-button"
      onclick={onDelete}
      disabled={$deleteItem.isPending}
    >
      <DeleteIcon />
    </button>
  </div>
</div>

<style>
  .throwable {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;

    padding: 0.5rem;
    overflow: hidden;
  }

  .throwable__content {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .throwable__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .throwable__image {
    width: 2.5rem;
    height: 2.5rem;
    object-fit: contain;
    background-color: #333;
    border-radius: 2rem;
  }

  .throwable__name {
    flex: 1;
    color: #fff;
    font-weight: bold;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .throw-button {
    padding: 0.5rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .throw-button:hover {
    background-color: #444;
  }
</style>
