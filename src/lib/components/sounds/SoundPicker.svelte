<script lang="ts">
  import type { SoundConfig } from "$shared/appData";
  import { Select } from "bits-ui";
  import SolarAltArrowRightBold from "~icons/solar/alt-arrow-right-bold";
  type Props = {
    id: string;
    name: string;
    items: SoundConfig[];
  };

  const { id, name, items }: Props = $props();

  const onSelectAll = () => {};
</script>

<select hidden {name} {id}></select>

<button>Select Sounds</button>

<Select.Root multiple {name} items={[]} let:ids>
  <Select.Trigger>
    <Select.Value />
  </Select.Trigger>

  <Select.Content>
    <button onclick={onSelectAll}>Select All</button>

    {#each items as item}
      <Select.Item value={item.id} label={item.name} let:isSelected>
        <Select.ItemIndicator>
          <SolarAltArrowRightBold />
        </Select.ItemIndicator>

        {item.name}
      </Select.Item>
    {/each}

    <Select.Arrow />
  </Select.Content>
</Select.Root>
