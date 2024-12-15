<script lang="ts">
  import getBackendURL from "$lib/utils/url";
  import type { Item } from "$shared/dataV2";
  import { fade, scale } from "svelte/transition";
  import { createItemsQuery } from "$lib/api/items";
  import { Dialog, Checkbox, Separator } from "bits-ui";

  import SearchInput from "../form/SearchInput.svelte";

  type Props = {
    selected: string[];
    onChangeSelect: (selected: string[]) => void;
  };

  const { selected, onChangeSelect }: Props = $props();

  let search = $state("");

  const itemsQuery = createItemsQuery();

  const items = $derived(filterOptionsSearch($itemsQuery.data ?? [], search));
  const selectedOptions = $derived(filterOptionsSelected(items, selected));

  function filterOptionsSelected(options: Item[], selected: string[]) {
    return options.filter((option) => selected.includes(option.id));
  }

  function filterOptionsSearch(options: Item[], search: string) {
    search = search.trim().toLowerCase();
    if (search.length < 1) return options;

    return options.filter((option) => {
      const name = option.name.trim().toLowerCase();
      return name.startsWith(search) || name.includes(search);
    });
  }

  const onSelectItem = (item: Item) => {
    if (selected.includes(item.id)) {
      onChangeSelect(selected.filter((id) => id !== item.id));
    } else {
      onChangeSelect([...selected, item.id]);
    }
  };

  const onToggleAll = () => {
    if (items.length > 0 && selected.length === items.length) {
      onChangeSelect([]);
    } else {
      onChangeSelect(items.map((item) => item.id));
    }
  };
</script>

{#if $itemsQuery.isLoading}
  Loading items...
{/if}

<Dialog.Root>
  <Dialog.Trigger type="button"
    >{selectedOptions.length > 0
      ? `${selectedOptions.length} Items selected`
      : "Select Items"}</Dialog.Trigger
  >
  <Dialog.Portal>
    <Dialog.Overlay transition={fade} transitionConfig={{ duration: 150 }} />
    <Dialog.Content transition={scale}>
      <Dialog.Title>Select Items</Dialog.Title>

      <Dialog.Description class="text-sm text-foreground-alt">
        Choose which items will be thrown
      </Dialog.Description>

      <Separator.Root />

      <div class="selection">
        <Checkbox.Root
          id="terms"
          aria-labelledby="terms-label"
          checked={items.length > 0 && selected.length === items.length}
          onCheckedChange={onToggleAll}
        >
          <Checkbox.Indicator let:isChecked>
            {#if isChecked}
              <span>&#10003;</span>
            {/if}
          </Checkbox.Indicator>
        </Checkbox.Root>

        <SearchInput bind:value={search} placeholder="Search" />
      </div>

      <div class="throwable-table-wrapper">
        <div class="items">
          {#each items as item (item.id)}
            <div class="item">
              <div class="item-column item-column--checkbox">
                <Checkbox.Root
                  id="terms"
                  aria-labelledby="terms-label"
                  checked={selected.includes(item.id)}
                  onCheckedChange={() => onSelectItem(item)}
                >
                  <Checkbox.Indicator let:isChecked>
                    {#if isChecked}
                      <span>&#10003;</span>
                    {/if}
                  </Checkbox.Indicator>
                </Checkbox.Root>
              </div>

              <div class="item-column item-column--preview">
                <div class="throwable__image-wrapper">
                  <img
                    class="throwable__image"
                    src={getBackendURL(item.image.src)}
                    alt="Throwable"
                  />
                </div>
              </div>

              <div class="item-column item-column--name">{item.name}</div>
            </div>
          {/each}
        </div>
      </div>

      <div data-dialog-actions>
        <Dialog.Close>
          <span class="sr-only">Close</span>
        </Dialog.Close>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

{#if selectedOptions.length > 0}
  <div class="selected">
    <p class="selected__title">Selected Items</p>

    <div class="grid-wrapper">
      <div class="grid">
        {#each selectedOptions as option}
          <li class="grid-item">
            <div class="grid-item__image throwable__image-wrapper">
              <img
                class="throwable__image"
                src={getBackendURL(option.image.src)}
                alt="Throwable"
              />
            </div>

            <p class="grid-item__name">{option.name}</p>
          </li>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .selected {
    margin-top: 0.5rem;
    display: flex;
    gap: 1rem;
    flex-flow: column;
    background-color: #333;
    padding: 1rem;
  }

  .selection {
    display: flex;
    gap: 1rem;
    align-items: center;
    padding-left: 1rem;
    padding-right: 1rem;
    padding-bottom: 1rem;
  }

  .items {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }

  .selected__title {
    color: #fff;
    font-weight: bold;
  }

  .grid-wrapper {
    max-height: 12rem;
    overflow: auto;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 0.5rem;
  }

  .grid-item {
    display: flex;
    gap: 1rem;
    align-items: center;
    width: 100%;
    overflow: hidden;
  }

  .grid-item__name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .throwable__image {
    width: 2.5rem;
    height: 2.5rem;
    object-fit: contain;
    background-color: #333;
    border-radius: 2rem;
  }

  .throwable-table-wrapper {
    padding: 1rem;
    max-height: 300px;
    overflow-y: auto;
    width: 100%;
    min-height: 300px;
  }

  .item {
    display: flex;
    flex-flow: row;
    gap: 1rem;
    align-items: center;
  }
</style>
