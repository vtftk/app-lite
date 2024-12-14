<script lang="ts" generics="T extends { value: string, label: string }">
  import { Select } from "bits-ui";
  import FormErrorLabel from "./FormErrorLabel.svelte";
  import type { Snippet } from "svelte";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";

  type Props<T> = {
    id: string;
    name: string;
    label: string;
    description?: string;

    items: T[];
    item: Snippet<[T]>;

    multiple?: boolean;

    selected: any;
    onChangeSelected: (value: any) => void;
  };

  const {
    id,
    name,
    label,
    description,
    items,
    item,
    multiple,
    selected: selectedValues,
    onChangeSelected,
  }: Props<T> = $props();

  let open = $state(false);

  function getSelection(items: T[], multiple: boolean, selected: any): any {
    if (multiple) {
      return items
        .filter((item) => selected.includes(item.value))
        .map((item) => ({
          value: item.value,
          label: item.label,
        }));
    } else {
      const item = items.find((item) => item.value === selected);
      if (item === undefined) return item;
      return {
        value: item.value,
        label: item.label,
      };
    }
  }
</script>

<div class="form-input">
  <label for={id}>{label}</label>
  <Select.Root
    {multiple}
    onOpenChange={(value) => {
      open = value;
    }}
    selected={getSelection(items, multiple ?? false, selectedValues)}
    onSelectedChange={(selection) => {
      if (selection === undefined) {
        onChangeSelected(multiple ? [] : undefined);
      } else if (Array.isArray(selection)) {
        onChangeSelected(selection.map((item) => item.value));
      } else {
        onChangeSelected(selection.value);
      }
    }}
  >
    <Select.Trigger class="btn">
      {#if open}
        <SolarAltArrowUpBold />
      {:else}
        <SolarAltArrowDownBold />
      {/if}
      <Select.Value />
    </Select.Trigger>

    <Select.Content sideOffset={8}>
      {#each items as value}
        <Select.Item value={value.value} label={value.label}>
          <Select.ItemIndicator />
          {@render item(value)}
        </Select.Item>
      {/each}

      <Select.Arrow />
    </Select.Content>

    <Select.Input {name} {id} data-felte-keep-on-remove />
  </Select.Root>

  {#if description}
    <p class="description">{description}</p>
  {/if}

  <FormErrorLabel {name} />
</div>

<style>
  .form-input {
    display: inline-flex;
    flex-flow: column;
  }

  .form-input label {
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }

  .form-input input {
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .description {
    font-size: 0.8rem;
    color: #999;
    margin-top: 0.5rem;
  }
</style>
