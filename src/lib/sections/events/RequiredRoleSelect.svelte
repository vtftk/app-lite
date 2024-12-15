<script lang="ts">
  import { MinimumRequiredRole } from "$shared/appData";
  import FormSelect from "$lib/components/form/FormSelect.svelte";

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
      description: "Anyone can redeem",
    },
    {
      value: MinimumRequiredRole.Vip,
      label: "VIP",
      description: "Must be VIP, Moderator, or the streamer to redeem",
    },
    {
      value: MinimumRequiredRole.Mod,
      label: "Moderator",
      description: "Must be moderator or the streamer to redeem",
    },
    {
      value: MinimumRequiredRole.Broadcaster,
      label: "Broadcaster",
      description: "Must be the streamer to redeem",
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
