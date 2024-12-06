<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import {
    EYES_MODE_VALUES,
    EyesMode,
    THROW_DIRECTION_VALUES,
    ThrowDirection,
    type AppData,
    type UserScriptConfig,
  } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import CodeEditor from "$lib/components/scripts/CodeEditor.svelte";
  import exampleCode from "../../../../script/example.js?raw";
  import FormCheckbox from "$lib/components/form/FormCheckbox.svelte";
  import { Tabs } from "bits-ui";
  import SolarCodeSquareBoldDuotone from "~icons/solar/code-square-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import { minMax } from "$lib/utils/validation";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import { toast } from "svelte-sonner";

  type Props = {
    existing?: UserScriptConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const throwablesSchema = z.object({
    duration: z.number(),
    spin_speed: minMax,
    throw_angle: minMax,
    direction: z.enum(THROW_DIRECTION_VALUES),
    impact_delay: z.number(),
    item_scale: minMax,
  });

  const modelSchema = z.object({
    model_return_time: z.number(),
    eyes_on_hit: z.enum(EYES_MODE_VALUES),
  });

  const soundsSchema = z.object({
    global_volume: z.number(),
  });

  const vtubeStudioSchema = z.object({
    host: z.string(),
    port: z.number(),
  });

  const schema = z.object({
    throwables: throwablesSchema,
    model: modelSchema,
    sounds: soundsSchema,
    vtube_studio: vtubeStudioSchema,
  });

  type Schema = z.infer<typeof schema>;

  function createInitialValues(appData: AppData): Schema {
    const {
      throwables_config,
      model_config,
      sounds_config,
      vtube_studio_config,
    } = appData;

    return {
      throwables: {
        duration: throwables_config.duration,
        spin_speed: throwables_config.spin_speed,
        throw_angle: throwables_config.throw_angle,
        direction: throwables_config.direction,
        impact_delay: throwables_config.impact_delay,
        item_scale: throwables_config.item_scale,
      },
      model: {
        model_return_time: model_config.model_return_time,
        eyes_on_hit: model_config.eyes_on_hit,
      },
      sounds: {
        global_volume: sounds_config.global_volume,
      },
      vtube_studio: {
        host: vtube_studio_config.host,
        port: vtube_studio_config.port,
      },
    };
  }

  const { form, data, setFields } = createForm<z.infer<typeof schema>>({
    initialValues: createInitialValues($appData),

    // Validation and error reporting
    extend: [validator({ schema }), reporterDom()],

    async onSubmit(values) {
      const savePromise = save(values);

      toast.promise(savePromise, {
        loading: "Saving settings...",
        success: "Saved settings",
        error: "Failed to save settings",
      });

      await savePromise;
    },
  });

  async function save(values: Schema) {
    const { throwables, model, sounds, vtube_studio } = values;
    const _appData = $appData;

    await $appDataMutation.mutateAsync({
      ..._appData,
      throwables_config: {
        ..._appData.throwables_config,
        duration: throwables.duration,
        spin_speed: throwables.spin_speed,
        throw_angle: throwables.throw_angle,
        direction: throwables.direction,
        impact_delay: throwables.impact_delay,
        item_scale: throwables.item_scale,
      },
      model_config: {
        ..._appData.model_config,
        model_return_time: model.model_return_time,
        eyes_on_hit: model.eyes_on_hit,
      },
      sounds_config: {
        ..._appData.sounds_config,
        global_volume: sounds.global_volume,
      },

      vtube_studio_config: {
        ..._appData.vtube_studio_config,
        host: vtube_studio.host,
        port: vtube_studio.port,
      },
    });
  }

  const directionOptions = [
    {
      value: ThrowDirection.Random,
      label: "Random",
      description: "Randomly pick between the left and right side",
    },
    {
      value: ThrowDirection.LeftOnly,
      label: "Left Only",
      description: "Only throw from the left side",
    },
    {
      value: ThrowDirection.RightOnly,
      label: "Right Only",
      description: "Only throw from the right side",
    },
  ];

  const eyesOptions = [
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
</script>

<form use:form class="container">
  {#snippet actions()}
    <button type="submit" class="btn">Save</button>
  {/snippet}

  <PageLayoutList
    title="Settings"
    description="Configuration for the entire app"
    {actions}
  >
    <div class="container">
      <Tabs.Root>
        <Tabs.List>
          <Tabs.Trigger value="throwables">
            <SolarSettingsBoldDuotone /> Throwables
          </Tabs.Trigger>
          <Tabs.Trigger value="sounds">
            <SolarSettingsBoldDuotone /> Sounds
          </Tabs.Trigger>
          <Tabs.Trigger value="vtube_studio">
            <SolarSettingsBoldDuotone /> VTube Studio
          </Tabs.Trigger>
          <Tabs.Trigger value="model">
            <SolarSettingsBoldDuotone /> VTuber Model
          </Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="throwables">
          <div class="settings">
            <section class="section">
              <div class="section__head">
                <h2>Duration and delay</h2>
                <p></p>
              </div>

              <FormNumberInput
                id="throwables.duration"
                name="throwables.duration"
                label="Duration"
                description=" Total time that it should take for a thrown item to hit the target"
              />

              <FormNumberInput
                id="throwables.impact_delay"
                name="throwables.impact_delay"
                label="Impact Delay"
                description="Delay before the impact is registered"
              />

              <div class="section__head">
                <h2>Spin Speed</h2>
                <p></p>
              </div>

              <!-- Spin speed -->
              <div class="row">
                <FormNumberInput
                  id="throwables.spin_speed.min"
                  name="throwables.spin_speed.min"
                  label="Minimum Spin Speed"
                  description="Minimum speed an item can spin at"
                />

                <FormNumberInput
                  id="throwables.spin_speed.max"
                  name="throwables.spin_speed.max"
                  label="Maximum Spin Speed"
                  description="Maximum speed an item can spin at"
                />
              </div>

              <div class="section__head">
                <h2>Angle and direction</h2>
                <p></p>
              </div>

              {#snippet directionItem(item: (typeof directionOptions)[0])}
                <div class="text-stack">
                  <p class="text-stack--top">{item.label}</p>
                  <p class="text-stack--bottom">{item.description}</p>
                </div>
              {/snippet}

              <FormSelect
                id="throwables.direction"
                name="throwables.direction"
                label="Direction"
                description="Which directions the items should come from"
                items={directionOptions}
                item={directionItem}
                selected={$data.throwables.direction}
                onChangeSelected={(selected) => {
                  setFields("throwables.direction", selected);
                }}
              />

              <!-- Throw angle -->
              <div class="row">
                <FormNumberInput
                  id="throwables.throw_angle.min"
                  name="throwables.throw_angle.min"
                  label="Minimum Throw Angle"
                  description="Minimum angle an item will be throw at"
                />

                <FormNumberInput
                  id="throwables.throw_angle.max"
                  name="throwables.throw_angle.max"
                  label="Maximum Throw Angle"
                  description="Maximum angle an item will be throw at"
                />
              </div>

              <div class="section__head">
                <h2>Scale</h2>
                <p></p>
              </div>

              <!-- Item scale -->
              <div class="row">
                <FormNumberInput
                  id="throwables.item_scale.min"
                  name="throwables.item_scale.min"
                  label="Minimum Scale"
                  description="Minimum scale applied to an item"
                />

                <FormNumberInput
                  id="throwables.item_scale.max"
                  name="throwables.item_scale.max"
                  label="Maximum Scale"
                  description="Maximum scale applied to an item"
                />
              </div>
            </section>
          </div>
        </Tabs.Content>
        <Tabs.Content value="sounds">
          <div class="settings">
            <section class="section">
              <div class="section__head">
                <h2>Volume</h2>
                <p></p>
              </div>

              <FormNumberInput
                id="sounds.global_volume"
                name="sounds.global_volume"
                label="Global Volume"
                description="Overall volume of all sounds, including impact sounds"
              />

              <!-- TODO: Sound alerts volume, impact sound volume -->
            </section>
          </div>
        </Tabs.Content>
        <Tabs.Content value="vtube_studio">
          <div class="settings">
            <section class="section">
              <div class="section__head">
                <h2>API Settings</h2>
                <p>Details for the VTube Studio API</p>
              </div>

              <div class="row row-ll">
                <FormTextInput
                  id="vtube_studio.host"
                  name="vtube_studio.host"
                  label="Host"
                  description="Host to use when connecting to VTube Studio"
                />

                <button
                  type="button"
                  class="btn"
                  onclick={() => {
                    setFields("vtube_studio.host", "localhost");
                  }}>Default</button
                >
              </div>

              <FormNumberInput
                id="vtube_studio.port"
                name="vtube_studio.port"
                label="Port"
                description="Port that the VTube Studio API is running on"
              />
            </section>
          </div>
        </Tabs.Content>
        <Tabs.Content value="model">
          <div class="settings">
            <section class="section">
              <div class="section__head">
                <h2>Model Settings</h2>
                <p></p>
              </div>

              <FormNumberInput
                id="model.model_return_time"
                name="model.model_return_time"
                label="Return Time"
                description="Time it takes for the model to return to its original position after being hit"
              />

              {#snippet eyeOptionItem(item: (typeof eyesOptions)[0])}
                <div class="text-stack">
                  <p class="text-stack--top">{item.label}</p>
                  <p class="text-stack--bottom">{item.description}</p>
                </div>
              {/snippet}

              <FormSelect
                id="model.eyes_on_hit"
                name="model.eyes_on_hit"
                label="Eyes On Hit"
                description="How the model eyes should react to being hit"
                items={eyesOptions}
                item={eyeOptionItem}
                selected={$data.model.eyes_on_hit}
                onChangeSelected={(selected) => {
                  setFields("model.eyes_on_hit", selected);
                }}
              />
            </section>
          </div>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  </PageLayoutList>
</form>

<style>
  .settings {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    padding: 0.5rem;
  }

  .section {
    display: flex;
    flex-flow: column;

    border: 1px solid #333;
    padding: 1rem;
    gap: 1rem;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    align-items: center;
    justify-content: center;
  }

  .row-ll {
    grid-template-columns: 3fr 1fr;
  }

  .container {
    position: relative;
    flex: auto;
    overflow: hidden;

    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    height: 100%;
  }

  .container :global([data-tabs-root]) {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .container :global([data-tabs-content]) {
    position: relative;
    flex: auto;
    overflow: auto;
    flex-flow: column;
    border: 1px solid #333;
  }

  .section__head {
    padding-bottom: 1rem;
    border-bottom: 1px solid #333;
  }

  .section__head h2 {
    color: #fff;
    font-size: 1.25rem;
    margin-bottom: 0.25rem;
  }

  .section__head p {
    color: #ccc;
  }
</style>
