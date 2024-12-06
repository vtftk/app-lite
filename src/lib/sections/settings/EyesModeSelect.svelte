<script lang="ts">
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import { EyesMode } from "$shared/appData";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    selected: EyesMode;
    onChangeSelected: (value: EyesMode) => void;
  };

  const { id, name, label, description, selected, onChangeSelected }: Props =
    $props();

  const options = [
    {
      value: EyesMode.Unchanged,
      label: "Unchanged",
      description: "Don't change eyes when hit",
    },
    {
      value: EyesMode.Opened,
      label: "Open",
      description: "Open model eyes when hit",
    },
    {
      value: EyesMode.Closed,
      label: "Close",
      description: "Close model eyes when hit",
    },
  ];

  type Option = (typeof options)[0];
</script>

{#snippet item(item: Option)}
  <div class="text-stack">
    <p class="text-stack--top">{item.label}</p>
    <p class="text-stack--bottom">{item.description}</p>
  </div>
{/snippet}

<FormSelect
  {id}
  {name}
  {label}
  {description}
  items={options}
  {item}
  {selected}
  {onChangeSelected}
/>
