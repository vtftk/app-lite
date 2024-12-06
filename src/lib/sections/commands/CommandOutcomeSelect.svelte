<script lang="ts">
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import { CommandOutcomeType, EyesMode } from "$shared/appData";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    selected: CommandOutcomeType;
    onChangeSelected: (value: CommandOutcomeType) => void;
  };

  const { id, name, label, description, selected, onChangeSelected }: Props =
    $props();

  const options = [
    {
      value: CommandOutcomeType.Template,
      label: "Template",
      description: "Message with support for basic templating",
    },
    {
      value: CommandOutcomeType.Script,
      label: "Script",
      description: "Custom JavaScript script command",
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
