<script lang="ts">
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import { EventTriggerType, EyesMode } from "$shared/appData";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    selected: EventTriggerType;
    onChangeSelected: (value: EventTriggerType) => void;
  };

  const { id, name, label, description, selected, onChangeSelected }: Props =
    $props();

  const options = [
    {
      value: EventTriggerType.Redeem,
      label: "Redeem",
      description: "Trigger when a twitch reward is redeemed",
    },
    {
      value: EventTriggerType.Command,
      label: "Command",
      description: "Trigger when a specific command message is sent",
    },
    {
      value: EventTriggerType.Follow,
      label: "Follow",
      description: "Trigger when a user follows the channel",
    },
    {
      value: EventTriggerType.Subscription,
      label: "Subscription",
      description: "Trigger when someone subscribes",
    },
    {
      value: EventTriggerType.GiftedSubscription,
      label: "Gifted Subscription",
      description: "Trigger when someone gifts a subscription",
    },
    {
      value: EventTriggerType.Bits,
      label: "Bits",
      description: "Trigger when someone cheers bits",
    },
    {
      value: EventTriggerType.Raid,
      label: "Raid",
      description: "Trigger when a raid occurs",
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
