<script lang="ts">
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import {
    EventTriggerType,
    EyesMode,
    MinimumRequiredRole,
  } from "$shared/appData";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    selected: MinimumRequiredRole;
    onChangeSelected: (value: MinimumRequiredRole) => void;
  };

  const { id, name, label, description, selected, onChangeSelected }: Props =
    $props();

  const options = [
    {
      value: MinimumRequiredRole.None,
      label: "None",
      description: "No minimum requirement",
    },
    {
      value: MinimumRequiredRole.Vip,
      label: "VIP",
      description: "Require VIP or greater to redeem",
    },
    {
      value: MinimumRequiredRole.Mod,
      label: "Moderator",
      description: "Require Moderator or greater to redeem",
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
