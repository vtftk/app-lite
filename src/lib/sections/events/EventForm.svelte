<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";

  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import {
    BitsAmountType,
    EventOutcomeType,
    EventTriggerType,
    MINIMUM_REQUIRED_ROLE_VALUES,
    MinimumRequiredRole,
    ThrowableDataType,
    type EventConfig,
    type EventTrigger,
  } from "$shared/appData";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormCheckbox from "$lib/components/form/FormCheckbox.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import TwitchRedeemSelect from "../twitch/TwitchRedeemSelect.svelte";
  import HotkeySelect from "./HotkeySelect.svelte";
  import { goto } from "$app/navigation";
  import ThrowablePicker from "$lib/components/throwable/ThrowablePicker.svelte";

  type Props = {
    existing?: EventConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const triggerSchema = z.discriminatedUnion("type", [
    z.object({
      type: z.literal(EventTriggerType.Redeem),
      reward_id: z.string(),
    }),
    z.object({
      type: z.literal(EventTriggerType.Command),
      message: z.string(),
    }),
    z.object({
      type: z.literal(EventTriggerType.Follow),
    }),
    z.object({
      type: z.literal(EventTriggerType.Subscription),
    }),
    z.object({
      type: z.literal(EventTriggerType.GiftedSubscription),
    }),
    z.object({
      type: z.literal(EventTriggerType.Bits),
      min_bits: z.number(),
    }),
    z.object({
      type: z.literal(EventTriggerType.Raid),
      min_raiders: z.number(),
    }),
  ]);

  type TriggerSchema = z.infer<typeof triggerSchema>;

  const throwableDataSchema = z.discriminatedUnion("type", [
    z.object({
      type: z.literal(ThrowableDataType.Throw),
      throwable_ids: z.array(z.string()),
      amount: z.number(),
    }),
    z.object({
      type: z.literal(ThrowableDataType.Barrage),
      throwable_ids: z.array(z.string()),
      amount_per_throw: z.number(),
      frequency: z.number(),
      amount: z.number(),
    }),
  ]);

  type ThrowableDataSchema = z.infer<typeof throwableDataSchema>;

  const outcomeSchema = z.discriminatedUnion("type", [
    z.object({
      type: z.literal(EventOutcomeType.ThrowBits),
      _1: z.string().nullable(),
      _100: z.string().nullable(),
      _1000: z.string().nullable(),
      _5000: z.string().nullable(),
      _10000: z.string().nullable(),
      amount: z.discriminatedUnion("type", [
        z.object({ type: z.literal(BitsAmountType.Fixed), amount: z.number() }),
        z.object({
          type: z.literal(BitsAmountType.Dynamic),
          max_amount: z.number(),
        }),
      ]),
    }),
    z.object({
      type: z.literal(EventOutcomeType.Throwable),
      data: throwableDataSchema,
    }),

    z.object({
      type: z.literal(EventOutcomeType.TriggerHotkey),
      hotkey_id: z.string(),
    }),
    z.object({
      type: z.literal(EventOutcomeType.PlaySound),
      sound_id: z.string(),
    }),
  ]);

  type OutcomeSchema = z.infer<typeof outcomeSchema>;

  const schema = z.object({
    name: z.string().min(1, "Name is required"),
    enabled: z.boolean(),

    trigger: triggerSchema,
    outcome: outcomeSchema,

    require_role: z.enum(MINIMUM_REQUIRED_ROLE_VALUES),
    cooldown: z.number(),
    outcome_delay: z.number(),
  });

  type Schema = z.infer<typeof schema>;

  const defaultData: Schema = {
    name: "",
    enabled: true,
    trigger: {
      type: EventTriggerType.Redeem,
      reward_id: "",
    },
    outcome: {
      type: EventOutcomeType.Throwable,
      data: {
        type: ThrowableDataType.Throw,
        throwable_ids: [],
        amount: 1,
      },
    },
    require_role: MinimumRequiredRole.None,
    cooldown: 0,
    outcome_delay: 0,
  };

  const { form, data, setFields } = createForm<Schema>({
    initialValues: existing ? { ...existing } : defaultData,
    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      const eventConfig: EventConfig = {
        id: existing ? existing.id : self.crypto.randomUUID(),
        name: values.name,
        enabled: values.enabled,
        trigger: values.trigger,
        outcome: values.outcome,
        cooldown: values.cooldown,
        require_role: values.require_role,
        outcome_delay: values.outcome_delay,
      };

      if (existing) {
        await $appDataMutation.mutateAsync({
          ...$appData,
          events: $appData.events.map((event) => {
            if (event.id === existing.id) {
              return eventConfig;
            }

            return event;
          }),
        });
      } else {
        await $appDataMutation.mutateAsync({
          ...$appData,
          events: [...$appData.events, eventConfig],
        });
      }

      goto("/events");
    },
  });

  const requiredRoles = [
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

  const triggerOptions = [
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

  const outcomeOptions = $derived([
    // Only include throw bits option when input type is bits
    ...($data.trigger.type === EventTriggerType.Bits
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
  ]);

  const throwableDataOptions = [
    {
      value: ThrowableDataType.Throw,
      label: "Throw",
      description: "Throw selected number of items all at once",
    },
    {
      value: ThrowableDataType.Barrage,
      label: "Barrage",
      description: "Throw items as a barrage of multiple thrown items",
    },
  ];

  const soundOptions = $derived(
    $appData.sounds.map((sound) => ({
      value: sound.id,
      label: sound.name,
    }))
  );

  function getTriggerDefaults(type: EventTriggerType): TriggerSchema {
    switch (type) {
      case EventTriggerType.Redeem:
        return { type: EventTriggerType.Redeem, reward_id: "" };
      case EventTriggerType.Command:
        return { type: EventTriggerType.Command, message: "" };
      case EventTriggerType.Follow:
        return { type: EventTriggerType.Follow };
      case EventTriggerType.Subscription:
        return { type: EventTriggerType.Subscription };
      case EventTriggerType.GiftedSubscription:
        return { type: EventTriggerType.GiftedSubscription };
      case EventTriggerType.Bits:
        return { type: EventTriggerType.Bits, min_bits: 1 };
      case EventTriggerType.Raid:
        return { type: EventTriggerType.Raid, min_raiders: 1 };
    }
  }

  function getOutcomeDefaults(type: EventOutcomeType): OutcomeSchema {
    switch (type) {
      case EventOutcomeType.ThrowBits:
        return {
          type: EventOutcomeType.ThrowBits,
          _1: null,
          _100: null,
          _1000: null,
          _10000: null,
          _5000: null,
          amount: {
            type: BitsAmountType.Dynamic,
            max_amount: 20,
          },
        };
      case EventOutcomeType.Throwable:
        return {
          type: EventOutcomeType.Throwable,
          data: {
            type: ThrowableDataType.Throw,
            amount: 1,
            throwable_ids: [],
          },
        };
      case EventOutcomeType.TriggerHotkey:
        return {
          type: EventOutcomeType.TriggerHotkey,
          hotkey_id: "",
        };
      case EventOutcomeType.PlaySound:
        return {
          type: EventOutcomeType.PlaySound,
          sound_id: "",
        };
    }
  }

  function onChangeTriggerType(type: EventTriggerType) {
    const defaults = getTriggerDefaults(type);

    // Reset invalid outcomes
    if (
      type !== EventTriggerType.Bits &&
      $data.outcome.type === EventOutcomeType.ThrowBits
    ) {
      setFields(
        "outcome",
        getOutcomeDefaults(EventOutcomeType.Throwable),
        true
      );
    }

    setFields("trigger", defaults, true);
  }

  function onChangeOutcomeType(type: EventOutcomeType) {
    const defaults = getOutcomeDefaults(type);
    setFields("outcome", defaults, true);
  }

  function getThrowableDataDefaults(
    type: ThrowableDataType
  ): ThrowableDataSchema {
    switch (type) {
      case ThrowableDataType.Throw:
        return { type: ThrowableDataType.Throw, amount: 1, throwable_ids: [] };
      case ThrowableDataType.Barrage:
        return {
          type: ThrowableDataType.Barrage,
          amount: 1,
          amount_per_throw: 1,
          frequency: 100,
          throwable_ids: [],
        };
    }
  }

  function onChangeThrowableDataType(type: ThrowableDataType) {
    const defaults = getThrowableDataDefaults(type);
    setFields("outcome.data", defaults, true);
  }
</script>

<form use:form>
  <!-- Base options -->

  <FormTextInput id="name" name="name" label="Name" />
  <FormCheckbox
    id="enabled"
    name="enabled"
    label="Enabled"
    checked={$data.enabled}
    onChecked={(checked) => {
      setFields("enabled", checked, true);
    }}
  />

  <!-- Trigger options -->

  {#snippet eventTriggerTypeItem(item: (typeof triggerOptions)[0])}
    <div class="text-stack">
      <p class="text-stack--top">{item.label}</p>
      <p class="text-stack--bottom">{item.description}</p>
    </div>
  {/snippet}

  <FormSelect
    id="trigger.type"
    name="trigger.type"
    label="Event Trigger"
    items={triggerOptions}
    item={eventTriggerTypeItem}
    selected={$data.trigger.type}
    onChangeSelected={(selected) => {
      onChangeTriggerType(selected);
    }}
  />

  {#if $data.trigger.type === EventTriggerType.Redeem}
    <TwitchRedeemSelect
      id="trigger.reward_id"
      name="trigger.reward_id"
      label="Reward"
      selected={$data.trigger.reward_id}
      onChangeSelected={(selected) =>
        setFields("trigger.reward_id", selected, true)}
    />
  {:else if $data.trigger.type === EventTriggerType.Command}
    <FormTextInput
      id="trigger.message"
      name="trigger.message"
      label="Command Message"
    />
  {:else if $data.trigger.type === EventTriggerType.Bits}
    <FormNumberInput
      id="trigger.min_bits"
      name="trigger.min_bits"
      label="Minimum Bits"
    />
  {:else if $data.trigger.type === EventTriggerType.Raid}
    <FormNumberInput
      id="trigger.min_raiders"
      name="trigger.min_raiders"
      label="Minimum Raiders"
    />
  {/if}

  <!-- Outcome options -->

  {#snippet eventOutcomeTypeItem(item: (typeof outcomeOptions)[0])}
    <div class="text-stack">
      <p class="text-stack--top">{item.label}</p>
      <p class="text-stack--bottom">{item.description}</p>
    </div>
  {/snippet}

  <FormSelect
    id="outcome.type"
    name="outcome.type"
    label="Event Outcome"
    items={outcomeOptions}
    item={eventOutcomeTypeItem}
    selected={$data.outcome.type}
    onChangeSelected={(selected) => {
      onChangeOutcomeType(selected);
    }}
  />

  {#if $data.outcome.type === EventOutcomeType.ThrowBits}
    <div></div>
  {:else if $data.outcome.type === EventOutcomeType.Throwable}
    {#snippet throwableDataTypeItem(item: (typeof throwableDataOptions)[0])}
      <div class="text-stack">
        <p class="text-stack--top">{item.label}</p>
        <p class="text-stack--bottom">{item.description}</p>
      </div>
    {/snippet}

    <FormSelect
      id="outcome.data.type"
      name="outcome.data.type"
      label="Throwable Type"
      items={throwableDataOptions}
      item={throwableDataTypeItem}
      selected={$data.outcome.data.type}
      onChangeSelected={(selected) => {
        onChangeThrowableDataType(selected);
      }}
    />

    {#if $data.outcome.data.type === ThrowableDataType.Throw}
      <div>
        <ThrowablePicker
          items={$appData.items}
          selected={$data.outcome.data.throwable_ids}
          onChangeSelect={(selected) => {
            setFields("outcome.data.throwable_ids", selected, true);
          }}
        />
        <FormNumberInput
          id="outcome.data.amount"
          name="outcome.data.amount"
          label="Total number of items to throw"
        />
      </div>
    {:else if $data.outcome.data.type === ThrowableDataType.Barrage}
      <div>
        <ThrowablePicker
          items={$appData.items}
          selected={$data.outcome.data.throwable_ids}
          onChangeSelect={(selected) => {
            setFields("outcome.data.throwable_ids", selected, true);
          }}
        />

        <FormNumberInput
          id="outcome.data.amount_per_throw"
          name="outcome.data.amount_per_throw"
          label="Amount of items per throw"
        />

        <FormNumberInput
          id="outcome.data.frequency"
          name="outcome.data.frequency"
          label="Frequency"
        />

        <FormNumberInput
          id="outcome.data.amount"
          name="outcome.data.amount"
          label="Total number of throws"
        />
      </div>
    {/if}
  {:else if $data.outcome.type === EventOutcomeType.TriggerHotkey}
    <HotkeySelect
      id="outcome.hotkey_id"
      name="outcome.hotkey_id"
      label="Hotkey"
      selected={$data.outcome.hotkey_id}
      onChangeSelected={(selected) =>
        setFields("outcome.hotkey_id", selected, true)}
    />
  {:else if $data.outcome.type === EventOutcomeType.PlaySound}
    {#snippet soundItem(item: (typeof soundOptions)[0])}
      <div class="text-stack">
        <p class="text-stack--top">{item.label}</p>
      </div>
    {/snippet}

    <FormSelect
      id="outcome.sound_id"
      name="outcome.sound_id"
      label="Sound"
      items={soundOptions}
      item={soundItem}
      selected={$data.outcome.sound_id}
      onChangeSelected={(selected) =>
        setFields("outcome.sound_id", selected, true)}
    />
  {/if}

  <!-- Cooldown and role requirements -->

  {#snippet requiredRoleItem(item: (typeof requiredRoles)[0])}
    <div class="text-stack">
      <p class="text-stack--top">{item.label}</p>
      <p class="text-stack--bottom">{item.description}</p>
    </div>
  {/snippet}

  <FormSelect
    id="require_role"
    name="require_role"
    label="Minimum Required Role"
    items={requiredRoles}
    item={requiredRoleItem}
    selected={$data.require_role}
    onChangeSelected={(selected) => setFields("require_role", selected, true)}
  />

  <FormNumberInput id="cooldown" name="cooldown" label="Cooldown" />
  <FormNumberInput
    id="outcome_delay"
    name="outcome_delay"
    label="Outcome Delay"
  />

  <button type="submit" class="btn">{existing ? "Save" : "Create"}</button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
