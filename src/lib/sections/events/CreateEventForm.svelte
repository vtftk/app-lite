<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import {
    EVENT_OUTCOME_NAMES,
    EVENT_OUTCOME_TYPES,
    EVENT_TRIGGER_NAMES,
    EVENT_TRIGGER_TYPES,
    EventOutcomeType,
    EventTriggerType,
    MINIMUM_REQUIRED_ROLE_NAMES,
    MINIMUM_REQUIRED_ROLE_VALUES,
    MinimumRequiredRole,
    type EventConfig,
    type EventOutcome,
    type EventTrigger,
  } from "$lib/api/types";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import TwitchRedeemSelect from "../twitch/TwitchRedeemSelect.svelte";
  import HotkeySelect from "./HotkeySelect.svelte";

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const schema = z.object({
    enabled: z.boolean(),
    triggerType: z.enum(EVENT_TRIGGER_TYPES),
    minimumRole: z.enum(MINIMUM_REQUIRED_ROLE_VALUES),
    eventOutcomeType: z.enum(EVENT_OUTCOME_TYPES),

    redeemRewardId: z.string(),

    commandMessage: z.string(),

    bitsMinBits: z.number(),
    bitsMaxItems: z.number(),

    raidMinRaiders: z.number(),
    raidMinItems: z.number(),
    raidMaxItems: z.number(),

    cooldown: z.number(),

    throwableThrowableId: z.string(),
    collectionCollectionId: z.string(),
    triggerHotkeyHotkeyId: z.string(),
  });

  const { form, data } = createForm<z.infer<typeof schema>>({
    initialValues: {
      enabled: true,
      triggerType: EventTriggerType.Redeem,
      minimumRole: MinimumRequiredRole.None,
      eventOutcomeType: EventOutcomeType.Random,

      redeemRewardId: "",

      commandMessage: "",

      bitsMinBits: 1,
      bitsMaxItems: 50,

      raidMinRaiders: 1,
      raidMinItems: 1,
      raidMaxItems: 50,

      cooldown: 500,

      throwableThrowableId: "",
      collectionCollectionId: "",
      triggerHotkeyHotkeyId: "",
    },
    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      let eventTrigger: EventTrigger;

      switch (values.triggerType) {
        case EventTriggerType.Redeem:
          eventTrigger = {
            type: EventTriggerType.Redeem,
            reward_id: values.redeemRewardId,
          };
          break;
        case EventTriggerType.Command:
          eventTrigger = {
            type: EventTriggerType.Command,
            message: values.commandMessage,
          };
          break;
        case EventTriggerType.Follow:
          eventTrigger = { type: EventTriggerType.Follow };
          break;
        case EventTriggerType.Subscription:
          eventTrigger = { type: EventTriggerType.Subscription };
          break;
        case EventTriggerType.GiftedSubscription:
          eventTrigger = { type: EventTriggerType.GiftedSubscription };
          break;
        case EventTriggerType.Bits:
          eventTrigger = {
            type: EventTriggerType.Bits,
            min_bits: values.bitsMinBits,
            max_throws: values.bitsMaxItems,
          };
          break;
        case EventTriggerType.Raid:
          eventTrigger = {
            type: EventTriggerType.Raid,
            min_raiders: values.raidMinRaiders,
            throws: {
              min: values.raidMinItems,
              max: values.raidMaxItems,
            },
          };
          break;
      }

      let eventOutcome: EventOutcome;

      switch (values.eventOutcomeType) {
        case EventOutcomeType.Random:
          eventOutcome = { type: EventOutcomeType.Random };
          break;
        case EventOutcomeType.RandomBarrage:
          eventOutcome = { type: EventOutcomeType.RandomBarrage };
          break;
        case EventOutcomeType.Throwable:
          eventOutcome = {
            type: EventOutcomeType.Throwable,
            throwable_id: values.throwableThrowableId,
          };
          break;
        case EventOutcomeType.Collection:
          eventOutcome = {
            type: EventOutcomeType.Collection,
            collection_id: values.collectionCollectionId,
          };
          break;
        case EventOutcomeType.TriggerHotkey:
          eventOutcome = {
            type: EventOutcomeType.TriggerHotkey,
            hotkey_id: values.triggerHotkeyHotkeyId,
          };
          break;
      }

      const eventConfig: EventConfig = {
        id: self.crypto.randomUUID(),
        enabled: values.enabled,
        trigger: eventTrigger,
        outcome: eventOutcome,
        cooldown: values.cooldown,
        require_role: values.minimumRole,
      };

      await $appDataMutation.mutateAsync({
        ...$appData,
        events: [...$appData.events, eventConfig],
      });
    },
  });
</script>

<form use:form>
  <div>
    <label for="enabled">Enabled</label>
    <input type="checkbox" name="enabled" id="enabled" />
  </div>

  <div>
    <label for="triggerType">Trigger Type</label>
    <select name="triggerType" id="triggerType">
      {#each EVENT_TRIGGER_TYPES as eventType}
        <option value={eventType}>{EVENT_TRIGGER_NAMES[eventType]}</option>
      {/each}
    </select>
  </div>

  {#if $data.triggerType === EventTriggerType.Redeem}
    <TwitchRedeemSelect name="redeemRewardId" id="redeemRewardId" />
  {:else if $data.triggerType === EventTriggerType.Command}
    <div>
      <label for="commandMessage">Command Message</label>
      <p>Message that should trigger the event</p>
      <input
        type="text"
        id="commandMessage"
        name="commandMessage"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="commandMessage-validation"
      />
      <p
        id="commandMessage-validation"
        data-felte-reporter-dom-for="commandMessage"
        aria-live="polite"
      ></p>
    </div>
  {:else if $data.triggerType === EventTriggerType.Bits}
    <div>
      <label for="bitsMinBits">Minimum Bits</label>
      <p>Minimum bits required to trigger</p>
      <input
        type="number"
        id="bitsMinBits"
        name="bitsMinBits"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="bitsMinBits-validation"
      />
      <p
        id="bitsMinBits-validation"
        data-felte-reporter-dom-for="bitsMinBits"
        aria-live="polite"
      ></p>
    </div>

    <div>
      <label for="bitsMaxItems">Max Items</label>
      <p>Max number of items to throw</p>
      <input
        type="number"
        id="bitsMaxItems"
        name="bitsMaxItems"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="bitsMaxItems-validation"
      />
      <p
        id="bitsMaxItems-validation"
        data-felte-reporter-dom-for="bitsMaxItems"
        aria-live="polite"
      ></p>
    </div>
  {:else if $data.triggerType === EventTriggerType.Raid}
    <div>
      <label for="raidMinRaiders">Min Raiders</label>
      <p>Minimum number of raiders required to trigger</p>
      <input
        type="number"
        id="raidMinRaiders"
        name="raidMinRaiders"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="raidMinRaiders-validation"
      />
      <p
        id="raidMinRaiders-validation"
        data-felte-reporter-dom-for="raidMinRaiders"
        aria-live="polite"
      ></p>
    </div>
    <div>
      <label for="raidMinItems">Min Items</label>
      <p>Minimum items to throw</p>
      <input
        type="number"
        id="raidMinItems"
        name="raidMinItems"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="raidMinItems-validation"
      />
      <p
        id="raidMinItems-validation"
        data-felte-reporter-dom-for="raidMinItems"
        aria-live="polite"
      ></p>
    </div>
    <div>
      <label for="raidMaxItems">Max Items</label>
      <p>Maximum items to throw</p>
      <input
        type="number"
        id="raidMaxItems"
        name="raidMaxItems"
        min="0"
        max="1"
        step="0.1"
        aria-describedby="raidMaxItems-validation"
      />
      <p
        id="raidMaxItems-validation"
        data-felte-reporter-dom-for="raidMaxItems"
        aria-live="polite"
      ></p>
    </div>
  {/if}

  <div>
    <label for="cooldown">Cooldown</label>
    <p>Cooldown between each trigger of this event</p>
    <input
      type="number"
      id="cooldown"
      name="cooldown"
      min="0"
      max="1"
      step="0.1"
      aria-describedby="cooldown-validation"
    />
    <p
      id="cooldown-validation"
      data-felte-reporter-dom-for="cooldown"
      aria-live="polite"
    ></p>
  </div>

  <div>
    <label for="minimumRole">Minimum role required</label>
    <p>Minimum twitch user access required to trigger this event</p>
    <select name="minimumRole" id="minimumRole">
      {#each MINIMUM_REQUIRED_ROLE_VALUES as roleType}
        <option value={roleType}>{MINIMUM_REQUIRED_ROLE_NAMES[roleType]}</option
        >
      {/each}
    </select>
  </div>

  <div>
    <label for="eventOutcomeType">Event Outcome</label>
    <select name="eventOutcomeType" id="eventOutcomeType">
      {#each EVENT_OUTCOME_TYPES as eventOutcomeType}
        <option value={eventOutcomeType}
          >{EVENT_OUTCOME_NAMES[eventOutcomeType]}</option
        >
      {/each}
    </select>
  </div>

  {#if $data.eventOutcomeType === EventOutcomeType.Throwable}
    <select name="throwableThrowableId" id="throwableThrowableId">
      <option value={"test"}>TEST</option>
    </select>
  {:else if $data.eventOutcomeType === EventOutcomeType.Collection}
    <select name="collectionCollectionId" id="collectionCollectionId">
      <option value={"test"}>TEST</option>
    </select>
  {:else if $data.eventOutcomeType === EventOutcomeType.TriggerHotkey}
    <HotkeySelect name="triggerHotkeyHotkeyId" id="triggerHotkeyHotkeyId" />
  {/if}

  <button type="submit">Create</button>
</form>
