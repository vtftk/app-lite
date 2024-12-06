<script lang="ts">
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import {
    EventOutcomeType,
    EventTriggerType,
    EyesMode,
  } from "$shared/appData";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    triggerType: EventTriggerType;

    selected: EventOutcomeType;
    onChangeSelected: (value: EventOutcomeType) => void;
  };

  const {
    id,
    name,
    label,
    description,
    triggerType,
    selected,
    onChangeSelected,
  }: Props = $props();

  const options = [
    // Only include throw bits option when input type is bits
    ...(triggerType === EventTriggerType.Bits
      ? [
          {
            value: EventOutcomeType.ThrowBits,
            label: "Throw Bits",
            description: "Throw bits",
          },
        ]
      : []),
    {
      value: EventOutcomeType.Throwable,
      label: "Throw Item",
      description: "Throw an item",
    },
    {
      value: EventOutcomeType.TriggerHotkey,
      label: "Trigger Hotkey",
      description: "Trigger a VTube studio hotkey",
    },
    {
      value: EventOutcomeType.PlaySound,
      label: "Play Sound",
      description: "Play a sound",
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
