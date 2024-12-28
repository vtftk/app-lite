<script lang="ts" generics="T extends { value: string, label: string }">
  import type { Snippet } from "svelte";

  import { Select } from "bits-ui";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";

  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props<T> = {
    id: string;
    name: string;
    label: string;
    description?: string;

    items: T[];
    item: Snippet<[T]>;

    selected: string[];
    onChangeSelected: (value: string[]) => void;
  };

  const {
    id,
    name,
    label,
    description,
    items,
    item,
    selected: selectedValues,
    onChangeSelected,
  }: Props<T> = $props();

  let open = $state(false);

  function getSelection(
    items: T[],
    selected: string[],
  ): { value: string; label: string }[] {
    return items
      .filter((item) => selected.includes(item.value))
      .map((item) => ({
        value: item.value,
        label: item.label,
      }));
  }

  const selection = $derived(getSelection(items, selectedValues));

  const selectedLabel = $derived.by(() => {
    const labels = selection.map((value) => value.label);

    if (labels.length < 1) {
      return undefined;
    }

    return labels.join(", ");
  });
</script>

<!-- eslint-disable @typescript-eslint/no-explicit-any -->

<div class="form-input">
  <label for={id}>{label}</label>
  <Select.Root
    type="multiple"
    onOpenChange={(value) => {
      open = value;
    }}
    value={selectedValues}
    onValueChange={(selection) => {
      onChangeSelected(selection);
    }}
  >
    <Select.Trigger class="btn">
      {#if open}
        <SolarAltArrowUpBold />
      {:else}
        <SolarAltArrowDownBold />
      {/if}

      {selectedLabel}
    </Select.Trigger>

    <Select.Portal>
      <Select.Content sideOffset={8}>
        {#each items as value}
          <Select.Item value={value.value} label={value.label}>
            {@render item(value)}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Portal>
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
