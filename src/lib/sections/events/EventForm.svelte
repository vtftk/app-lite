<script lang="ts">
  import { z } from "zod";
  import { Tabs } from "bits-ui";
  import { createForm } from "felte";
  import { toast } from "svelte-sonner";
  import { goto } from "$app/navigation";
  import reporterDom from "@felte/reporter-dom";
  import { minMax } from "$lib/utils/validation";
  import { validator } from "@felte/validator-zod";
  import { toastErrorMessage } from "$lib/utils/error";
  import BallIcon from "~icons/solar/basketball-bold-duotone";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import SolarBookBoldDuotone from "~icons/solar/book-bold-duotone";
  import SolarGiftBoldDuotone from "~icons/solar/gift-bold-duotone";
  import SolarCard2BoldDuotone from "~icons/solar/card-2-bold-duotone";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import { testEvent, createEvent, updateEvent } from "$lib/api/vevents";
  import SolarReorderBoldDuotone from "~icons/solar/reorder-bold-duotone";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import SolarKeyboardBoldDuotone from "~icons/solar/keyboard-bold-duotone";
  import SolarCardSendBoldDuotone from "~icons/solar/card-send-bold-duotone";
  import SolarHandMoneyBoldDuotone from "~icons/solar/hand-money-bold-duotone";
  import FormBoundCheckbox from "$lib/components/form/FormBoundCheckbox.svelte";
  import SolarBasketballBoldDuotone from "~icons/solar/basketball-bold-duotone";
  import ThrowablePicker from "$lib/components/throwable/ThrowablePicker.svelte";
  import SolarCardReciveBoldDuotone from "~icons/solar/card-recive-bold-duotone";
  import SolarBoltCircleBoldDuotone from "~icons/solar/bolt-circle-bold-duotone";
  import SolarTextSquareBoldDuotone from "~icons/solar/text-square-bold-duotone";
  import SolarSkateboardingBoldDuotone from "~icons/solar/skateboarding-bold-duotone";
  import SolarUsersGroupRoundedBoldDuotone from "~icons/solar/users-group-rounded-bold-duotone";
  import {
    type VEvent,
    type VEventData,
    SubscriptionTier,
  } from "$shared/dataV2";
  import SolarHeadphonesRoundSoundBoldDuotone from "~icons/solar/headphones-round-sound-bold-duotone";
  import SolarChecklistMinimalisticBoldDuotone from "~icons/solar/checklist-minimalistic-bold-duotone";
  import {
    BitsAmountType,
    EventOutcomeType,
    EventTriggerType,
    ThrowableDataType,
    MinimumRequiredRole,
    MINIMUM_REQUIRED_ROLE_VALUES,
  } from "$shared/appData";

  import SoundSelect from "./SoundSelect.svelte";
  import HotkeySelect from "./HotkeySelect.svelte";
  import EventExecutions from "./EventExecutions.svelte";
  import EventTriggerItem from "./EventTriggerItem.svelte";
  import EventOutcomeItem from "./EventOutcomeItem.svelte";
  import RequiredRoleSelect from "./RequiredRoleSelect.svelte";
  import TwitchRedeemSelect from "../twitch/TwitchRedeemSelect.svelte";
  import ThrowableDataTypeSelect from "./ThrowableDataTypeSelect.svelte";

  type Props = {
    existing?: VEvent;
  };

  const { existing }: Props = $props();

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

  const inputAmountConfigSchema = z.object({
    multiplier: z.number(),
    range: minMax,
  });
  type TriggerSchema = z.infer<typeof triggerSchema>;

  const throwableDataSchema = z.discriminatedUnion("type", [
    z.object({
      type: z.literal(ThrowableDataType.Throw),
      throwable_ids: z.array(z.string()),
      amount: z.number().default(1),
      use_input_amount: z.boolean().default(false),
      input_amount_config: inputAmountConfigSchema,
    }),
    z.object({
      type: z.literal(ThrowableDataType.Barrage),
      throwable_ids: z.array(z.string()),
      amount_per_throw: z.number(),
      frequency: z.number(),
      amount: z.number().default(1),
      use_input_amount: z.boolean().default(false),
      input_amount_config: inputAmountConfigSchema,
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

  const createDefaults: Schema = {
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
        use_input_amount: false,
        input_amount_config: {
          multiplier: 1,
          range: { min: 1, max: 1000 },
        },
      },
    },
    require_role: MinimumRequiredRole.None,
    cooldown: 0,
    outcome_delay: 0,
  };

  function createFromExisting(config: VEvent): Partial<Schema> {
    return {
      ...config,
    };
  }

  const { form, data, setFields } = createForm<Schema>({
    // Derive initial values
    initialValues: existing ? createFromExisting(existing) : createDefaults,

    // Validation and error reporting
    extend: [validator({ schema }), reporterDom()],

    onSubmit(values) {
      const savePromise = save(values);

      toast.promise(
        savePromise,
        existing
          ? {
              loading: "Saving event...",
              success: "Saved event",
              error: toastErrorMessage("Failed to save event"),
            }
          : {
              loading: "Creating event...",
              success: "Created event",
              error: toastErrorMessage("Failed to create event"),
            },
      );

      if (!existing) {
        goto("/events");
      }
    },
  });

  async function save(values: Schema) {
    if (existing) {
      await updateEvent({
        eventId: existing.id,
        update: {
          name: values.name,
          enabled: values.enabled,
          trigger: values.trigger,
          outcome: values.outcome,
          cooldown: values.cooldown,
          require_role: values.require_role,
          outcome_delay: values.outcome_delay,
        },
      });
    } else {
      await createEvent({
        name: values.name,
        enabled: values.enabled,
        trigger: values.trigger,
        outcome: values.outcome,
        cooldown: values.cooldown,
        require_role: values.require_role,
        outcome_delay: values.outcome_delay,
      });
    }
  }

  function getTriggerDefaults(type: EventTriggerType): TriggerSchema {
    switch (type) {
      case EventTriggerType.Redeem:
        return { type: EventTriggerType.Redeem, reward_id: "" };
      case EventTriggerType.Command:
        return { type: EventTriggerType.Command, message: "!test" };
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
            use_input_amount: false,
            input_amount_config: {
              multiplier: 1,
              range: { min: 1, max: 100 },
            },
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

  const EVENT_TRIGGERS_WITH_INPUT = [
    EventTriggerType.Bits,
    EventTriggerType.GiftedSubscription,
    EventTriggerType.Subscription,
    EventTriggerType.Raid,
  ];

  const EVENT_TRIGGER_INPUT_LABEL: Partial<
    Record<EventTriggerType, { label: string; description: string }>
  > = {
    [EventTriggerType.Bits]: {
      label: "Use bits amount",
      description: "Use the amount of bits for the amount of thrown items",
    },
    [EventTriggerType.GiftedSubscription]: {
      label: "Use total gifted subs",
      description:
        "Use the amount of gifted subscriptions for the amount of thrown items",
    },
    [EventTriggerType.Subscription]: {
      label: "Use total gifted subs",
      description:
        "Use the amount of months subscribed for the amount of thrown items",
    },
    [EventTriggerType.Raid]: {
      label: "Use raiders count",
      description: "Use the number of raiders for the amount of thrown items",
    },
  };

  const isEventTriggerWithInput = $derived(
    EVENT_TRIGGERS_WITH_INPUT.includes($data.trigger.type),
  );

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
        true,
      );
    }

    // Disable "use_input_amount" when trigger becomes a trigger that
    // does not produce an input amount
    if (
      $data.outcome.type === EventOutcomeType.Throwable &&
      !EVENT_TRIGGERS_WITH_INPUT.includes(type)
    ) {
      const tData = $data.outcome.data;

      if (
        (tData.type === ThrowableDataType.Throw ||
          tData.type === ThrowableDataType.Barrage) &&
        tData.use_input_amount
      ) {
        setFields("outcome.data.use_input_amount", false, true);
      }
    }

    setFields("trigger", defaults, true);
  }

  function onChangeOutcomeType(type: EventOutcomeType) {
    const defaults = getOutcomeDefaults(type);
    setFields("outcome", defaults, true);
  }

  function getThrowableDataDefaults(
    type: ThrowableDataType,
  ): ThrowableDataSchema {
    switch (type) {
      case ThrowableDataType.Throw:
        return {
          type: ThrowableDataType.Throw,
          amount: 1,
          throwable_ids: [],
          use_input_amount: false,
          input_amount_config: {
            multiplier: 1,
            range: { min: 1, max: 1000 },
          },
        };
      case ThrowableDataType.Barrage:
        return {
          type: ThrowableDataType.Barrage,
          amount: 50,
          amount_per_throw: 5,
          frequency: 100,
          throwable_ids: [],
          use_input_amount: false,
          input_amount_config: {
            multiplier: 1,
            range: { min: 1, max: 1000 },
          },
        };
    }
  }

  function onChangeThrowableDataType(type: ThrowableDataType) {
    const defaults = getThrowableDataDefaults(type);
    setFields("outcome.data", defaults, true);
  }

  const eventTriggerOptions = [
    {
      icon: SolarBoltCircleBoldDuotone,
      color: "purple",
      value: EventTriggerType.Redeem,
      label: "Redeem",
      description:
        "Event will be triggered when a specific channel points redeem is redeemed",
      content: redeemContent,
    },
    {
      icon: SolarTextSquareBoldDuotone,
      color: "red",
      value: EventTriggerType.Command,
      label: "Command",
      description:
        "Event will be triggered when a specific phrase appears at the start of a message",
      content: commandContent,
    },
    {
      icon: SolarUsersGroupRoundedBoldDuotone,
      color: "yellow",
      value: EventTriggerType.Follow,
      label: "Follow",
      description:
        "Event will be triggered when a user follows the twitch channel",
    },
    {
      icon: SolarCard2BoldDuotone,
      color: "green",
      value: EventTriggerType.Subscription,
      label: "Subscription",
      description:
        "Event will be triggered when a user purchases a subscription",
    },
    {
      icon: SolarGiftBoldDuotone,
      color: "blue",
      value: EventTriggerType.GiftedSubscription,
      label: "Gifted Subscription",
      description:
        "Event will be triggered when a user gifts any number of subscriptions",
    },
    {
      icon: SolarHandMoneyBoldDuotone,
      color: "purple",
      value: EventTriggerType.Bits,
      label: "Bits",
      description: "Event will trigger when bits are gifted to the channel",
      content: bitsContent,
    },
    {
      icon: SolarSkateboardingBoldDuotone,
      color: "red",
      value: EventTriggerType.Raid,
      label: "Raid",
      description:
        "Event will trigger when the channel is raided by another channel",
      content: raidContent,
    },
  ];

  const outcomeOptions = $derived([
    ...($data.trigger.type === EventTriggerType.Bits
      ? [
          {
            icon: SolarHandMoneyBoldDuotone,
            color: "green",
            value: EventOutcomeType.ThrowBits,
            label: "Throw Bits",
            description:
              "Only available when using the bits trigger, will throw bits",
          },
        ]
      : []),
    {
      icon: SolarBasketballBoldDuotone,
      color: "purple",
      value: EventOutcomeType.Throwable,
      label: "Throw Item",
      description: "Throw a random item from the specified collection",
      content: throwableOutcomeContent,
    },
    {
      icon: SolarKeyboardBoldDuotone,
      color: "red",
      value: EventOutcomeType.TriggerHotkey,
      label: "Trigger Hotkey",
      description: "Trigger a VTube studio hotkey",
      content: triggerHotkeyOutcomeContent,
    },
    {
      icon: SolarHeadphonesRoundSoundBoldDuotone,
      color: "yellow",
      value: EventOutcomeType.PlaySound,
      label: "Play Sound",
      description: "Play a sound from the available sounds",
      content: playSoundOutcomeContent,
    },
  ]);

  function onTest() {
    if (existing === undefined) return;

    let eventData: VEventData = {
      user: {
        id: "test_user",
        name: "test_user",
        display_name: "TestTwitchUser",
      },
    };

    switch ($data.outcome.type) {
      case EventOutcomeType.ThrowBits: {
        eventData = {
          user: {
            id: "test_user",
            name: "test_user",
            display_name: "TestTwitchUser",
          },
          bits: Math.floor(Math.random() * 10_000),
          anonymous: false,
          message: "Wooo bits!",
        };
        break;
      }
      case EventOutcomeType.Throwable: {
        switch ($data.trigger.type) {
          case EventTriggerType.Subscription:
            eventData = {
              user: {
                id: "test_user",
                name: "test_user",
                display_name: "TestTwitcwhUser",
              },
              tier: SubscriptionTier.Tier1,
              is_gift: Math.random() < 0.5,
            };
            break;
          case EventTriggerType.GiftedSubscription:
            eventData = {
              user: {
                id: "test_user",
                name: "test_user",
                display_name: "TestTwitchUser",
              },
              tier: SubscriptionTier.Tier1,
              cumulative_total: Math.floor(Math.random() * 12),
              anonymous: false,
              total: Math.floor(Math.random() * 100),
            };
            break;
          case EventTriggerType.Bits:
            eventData = {
              user: {
                id: "test_user",
                name: "test_user",
                display_name: "TestTwitchUser",
              },
              bits: Math.floor(Math.random() * 30_000),
              anonymous: false,
              message: "Test bits donation message",
            };
            break;
          case EventTriggerType.Raid:
            eventData = {
              user: {
                id: "test_user",
                name: "test_user",
                display_name: "TestTwitchUser",
              },
              viewers: Math.floor(Math.random() * 10_000),
            };
            break;
          default:
            break;
        }
        break;
      }
      default:
        break;
    }

    const throwPromise = testEvent(existing.id, eventData);

    toast.promise(throwPromise, {
      loading: "Sending test event...",
      success: "Tested event",
      error: toastErrorMessage("Failed to test event"),
    });
  }
</script>

{#snippet redeemContent()}
  {#if $data.trigger.type === EventTriggerType.Redeem}
    <TwitchRedeemSelect
      id="trigger.reward_id"
      name="trigger.reward_id"
      label="Reward"
      selected={$data.trigger.reward_id}
      onChangeSelected={(selected) =>
        setFields("trigger.reward_id", selected, true)}
      description="Choose the twitch redeem that will trigger this event"
    />
  {/if}
{/snippet}

{#snippet commandContent()}
  {#if $data.trigger.type === EventTriggerType.Command}
    <FormTextInput
      id="trigger.message"
      name="trigger.message"
      label="Command phrase"
      description="Triggers when a chat message starting with the provided phrase is received (e.g !test)"
    />
  {/if}
{/snippet}

{#snippet bitsContent()}
  {#if $data.trigger.type === EventTriggerType.Bits}
    <FormNumberInput
      id="trigger.min_bits"
      name="trigger.min_bits"
      label="Minimum Bits"
      description="Minimum number of bits that must be gifted to trigger"
    />
  {/if}
{/snippet}

{#snippet raidContent()}
  {#if $data.trigger.type === EventTriggerType.Raid}
    <FormNumberInput
      id="trigger.min_raiders"
      name="trigger.min_raiders"
      label="Minimum Raiders"
      description="Minimum number of people that must be apart of the raid to trigger"
    />
  {/if}
{/snippet}

{#snippet throwableOutcomeContent()}
  {#if $data.outcome.type === EventOutcomeType.Throwable}
    <ThrowableDataTypeSelect
      id="outcome.data.type"
      name="outcome.data.type"
      label="Throwable Type"
      selected={$data.outcome.data.type}
      onChangeSelected={(selected) => {
        onChangeThrowableDataType(selected);
      }}
    />

    {#if $data.outcome.data.type === ThrowableDataType.Throw}
      {#if isEventTriggerWithInput}
        {@const { label, description } =
          EVENT_TRIGGER_INPUT_LABEL[$data.trigger.type]!}
        <FormBoundCheckbox
          id="outcome.data.use_input_amount"
          name="outcome.data.use_input_amount"
          {label}
          {description}
        />
      {/if}

      {#if isEventTriggerWithInput && $data.outcome.data.use_input_amount}
        <FormNumberInput
          id="outcome.data.input_amount_config.multiplier"
          name="outcome.data.input_amount_config.multiplier"
          label="Multiplier"
          description="Multiplier applied against the amount"
          min={1}
          step={0.1}
          max={100}
        />
        <div class="throwable-config-grid">
          <FormNumberInput
            id="outcome.data.input_amount_config.range.min"
            name="outcome.data.input_amount_config.range.min"
            label="Minimum Amount"
            description="Minimum amount of items to throw"
            min={1}
            step={1}
            max={1000}
          />
          <FormNumberInput
            id="outcome.data.input_amount_config.range.max"
            name="outcome.data.input_amount_config.range.max"
            label="Maximum Amount"
            description="Maximum amount of items to throw"
            min={1}
            step={1}
            max={1000}
          />
        </div>
      {:else}
        <FormNumberInput
          id="outcome.data.amount"
          name="outcome.data.amount"
          label="Total number of items to throw"
          min={1}
        />
      {/if}

      <p>
        {$data.outcome.data.amount} random item{$data.outcome.data.amount > 1
          ? "s"
          : ""} will be chosen from your selection below and thrown
      </p>

      <ThrowablePicker
        selected={$data.outcome.data.throwable_ids}
        onChangeSelect={(selected) => {
          setFields("outcome.data.throwable_ids", selected, true);
        }}
      />
    {:else if $data.outcome.data.type === ThrowableDataType.Barrage}
      <div class="throwable-config-grid">
        <FormNumberInput
          id="outcome.data.amount_per_throw"
          name="outcome.data.amount_per_throw"
          label="Amount per throw"
          description="How many items to throw in each barrage"
          min={1}
        />

        <FormNumberInput
          id="outcome.data.frequency"
          name="outcome.data.frequency"
          label="Frequency"
          description="Time between each barrage of items (ms)"
          step={100}
          min={0}
          max={1000 * 60 * 60}
        />
      </div>

      {#if isEventTriggerWithInput}
        {@const { label, description } =
          EVENT_TRIGGER_INPUT_LABEL[$data.trigger.type]!}
        <FormBoundCheckbox
          id="outcome.data.use_input_amount"
          name="outcome.data.use_input_amount"
          {label}
          {description}
        />
      {/if}

      {#if isEventTriggerWithInput && $data.outcome.data.use_input_amount}
        <div class="throwable-config-grid">
          <FormNumberInput
            id="outcome.data.input_amount_config.multiplier"
            name="outcome.data.input_amount_config.multiplier"
            label="Multiplier"
            description="Multiplier applied against the amount"
            min={1}
            step={0.1}
            max={100}
          />
          <FormNumberInput
            id="outcome.data.input_amount_config.range.min"
            name="outcome.data.input_amount_config.range.min"
            label="Minimum Amount"
            description="Minimum amount of items to throw"
            min={1}
            step={1}
            max={1000}
          />
          <FormNumberInput
            id="outcome.data.input_amount_config.range.max"
            name="outcome.data.input_amount_config.range.max"
            label="Maximum Amount"
            description="Maximum amount of items to throw"
            min={1}
            step={1}
            max={1000}
          />
        </div>
      {:else}
        <FormNumberInput
          id="outcome.data.amount"
          name="outcome.data.amount"
          label="Total number of items to throw"
          description="Total number of items to throw for the whole barrage"
          min={1}
        />
      {/if}

      <p>
        {$data.outcome.data.amount_per_throw} random item{$data.outcome.data
          .amount > 1
          ? "s"
          : ""} will be chosen from your selection below and thrown every {$data
          .outcome.data.frequency}ms {$data.outcome.data.use_input_amount
          ? "until a maximum of " +
            $data.outcome.data.input_amount_config.range.max +
            " have been thrown based on the input "
          : "until a total of " + ($data.outcome.data.amount ?? 1)} item{$data
          .outcome.data.amount > 1
          ? "s"
          : ""} have been thrown
      </p>

      <ThrowablePicker
        selected={$data.outcome.data.throwable_ids}
        onChangeSelect={(selected) => {
          setFields("outcome.data.throwable_ids", selected, true);
        }}
      />
    {/if}
  {/if}
{/snippet}

{#snippet triggerHotkeyOutcomeContent()}
  {#if $data.outcome.type === EventOutcomeType.TriggerHotkey}
    <HotkeySelect
      id="outcome.hotkey_id"
      name="outcome.hotkey_id"
      label="Hotkey"
      selected={$data.outcome.hotkey_id}
      onChangeSelected={(selected) =>
        setFields("outcome.hotkey_id", selected, true)}
      description="Choose which VTube Studio hotkey to trigger"
    />
  {/if}
{/snippet}
{#snippet playSoundOutcomeContent()}
  {#if $data.outcome.type === EventOutcomeType.PlaySound}
    <SoundSelect
      id="outcome.sound_id"
      name="outcome.sound_id"
      label="Sound"
      selected={$data.outcome.sound_id}
      onChangeSelected={(selected) =>
        setFields("outcome.sound_id", selected, true)}
    />
  {/if}
{/snippet}

<form use:form>
  {#snippet actions()}
    {#if existing}
      <button type="button" class="btn" onclick={onTest}>
        <BallIcon /> Test
      </button>
    {/if}

    <button type="submit" class="btn">{existing ? "Save" : "Create"}</button>
    <a type="button" class="btn" href="/events">Back</a>
  {/snippet}

  <PageLayoutList
    title={existing ? "Edit Event" : "Create Event"}
    description={existing
      ? "Editing Event"
      : "Create an event that will trigger some outcome"}
    {actions}
  >
    <div class="content">
      <Tabs.Root let:value>
        <Tabs.List>
          <Tabs.Trigger value="details">
            <SolarBookBoldDuotone />
            Details
          </Tabs.Trigger>
          <Tabs.Trigger value="trigger">
            <SolarCardReciveBoldDuotone />
            Trigger
          </Tabs.Trigger>

          <Tabs.Trigger value="outcome">
            <SolarCardSendBoldDuotone />
            Outcome
          </Tabs.Trigger>
          <Tabs.Trigger value="cooldown">
            <SolarChecklistMinimalisticBoldDuotone />
            Requirements
          </Tabs.Trigger>
          {#if existing !== undefined}
            <Tabs.Trigger value="executions">
              <SolarReorderBoldDuotone /> Executions
            </Tabs.Trigger>
          {/if}
        </Tabs.List>
        <Tabs.Content value="details">
          <!-- Base options -->
          <FormSection
            title="Details"
            description="Basic details about the event"
          >
            <FormTextInput
              id="name"
              name="name"
              label="Name"
              placeholder="Example Event"
            />
            <FormBoundCheckbox
              id="enabled"
              name="enabled"
              label="Enabled"
              description="Whether this event will be triggered"
            />
          </FormSection>
        </Tabs.Content>
        <Tabs.Content value="trigger">
          <div class="event-trigger-grid">
            {#each eventTriggerOptions as option (option.value)}
              <EventTriggerItem
                icon={option.icon}
                color={option.color}
                label={option.label}
                description={option.description}
                selected={$data.trigger.type === option.value}
                onClick={() =>
                  $data.trigger.type !== option.value &&
                  onChangeTriggerType(option.value)}
                content={option.content}
                contentVisible={$data.trigger.type === option.value}
              />
            {/each}
          </div>
        </Tabs.Content>

        <Tabs.Content value="outcome">
          <div class="event-trigger-grid">
            {#each outcomeOptions as option (option.value)}
              <EventOutcomeItem
                icon={option.icon}
                color={option.color}
                label={option.label}
                description={option.description}
                selected={$data.outcome.type === option.value}
                onClick={() =>
                  $data.outcome.type !== option.value &&
                  onChangeOutcomeType(option.value)}
                content={option.content}
                contentVisible={$data.outcome.type === option.value}
              />
            {/each}
          </div>
        </Tabs.Content>
        <Tabs.Content value="cooldown">
          <!-- Cooldown and role requirements -->
          <FormSection
            title="Delays, cooldown, and requirements"
            description="Configure any delays, cooldown, or requirements on this events trigger"
          >
            <RequiredRoleSelect
              id="require_role"
              name="require_role"
              label="Minimum Required Role"
              selected={$data.require_role}
              onChangeSelected={(selected) =>
                setFields("require_role", selected, true)}
              description="Minimum required role the user triggering the event must have in order for the event to trigger"
            />

            <FormNumberInput
              id="cooldown"
              name="cooldown"
              label="Cooldown"
              description="Cooldown before this event can be triggered again (ms)"
              min={0}
              step={100}
            />
            <FormNumberInput
              id="outcome_delay"
              name="outcome_delay"
              label="Outcome Delay"
              description="Delay before this event will be triggered (ms)"
              min={0}
              step={100}
            />
          </FormSection>
        </Tabs.Content>
        {#if existing !== undefined}
          <Tabs.Content value="executions">
            {#if value === "executions"}
              <EventExecutions id={existing.id} />
            {/if}
          </Tabs.Content>
        {/if}
      </Tabs.Root>
    </div>
  </PageLayoutList>
</form>

<style>
  form {
    height: 100%;
  }

  .throwable-config-grid {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 1rem;
  }

  .content {
    position: relative;
    flex: auto;
    overflow: hidden;
    height: 100%;
  }

  .content :global([data-tabs-root]) {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .content :global([data-tabs-content]) {
    position: relative;
    flex: auto;
    overflow: auto;
    flex-flow: column;
    border: 1px solid #333;
    padding: 1rem;
  }

  .event-trigger-grid {
    display: grid;

    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
</style>
