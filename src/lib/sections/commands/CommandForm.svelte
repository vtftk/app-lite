<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import {
    CommandOutcomeType,
    MINIMUM_REQUIRED_ROLE_VALUES,
    MinimumRequiredRole,
    type CommandConfig,
  } from "$lib/api/types";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import CodeEditor from "$lib/components/scripts/CodeEditor.svelte";
  import exampleCode from "../../../../script/example_command.js?raw";
  import FormCheckbox from "$lib/components/form/FormCheckbox.svelte";
  import { Tabs } from "bits-ui";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import SolarCodeSquareBoldDuotone from "~icons/solar/code-square-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";

  type Props = {
    existing?: CommandConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const outcomeSchema = z.discriminatedUnion("type", [
    z.object({
      type: z.literal(CommandOutcomeType.Template),
      message: z.string(),
    }),
    z.object({
      type: z.literal(CommandOutcomeType.Script),
      script: z.string(),
    }),
  ]);

  type OutcomeSchema = z.infer<typeof outcomeSchema>;

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    command: z.string().min(1, "You must specify a command"),
    enabled: z.boolean(),
    outcome: outcomeSchema,
    require_role: z.enum(MINIMUM_REQUIRED_ROLE_VALUES),
    cooldown: z.number(),
  });

  type Schema = z.infer<typeof schema>;

  const { form, data, setFields, isDirty, setIsDirty } = createForm<
    z.infer<typeof schema>
  >({
    initialValues: existing
      ? {
          name: existing.name,
          command: existing.command,
          enabled: existing.enabled,
          outcome: existing.outcome,
          require_role: existing.require_role,
          cooldown: existing.cooldown,
        }
      : {
          name: "",
          command: "!test",
          enabled: true,
          outcome: getOutcomeDefaults(CommandOutcomeType.Script),
          require_role: MinimumRequiredRole.None,
          cooldown: 1000,
        },

    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      await save(values);
      goto("/commands");
    },
  });

  async function save(values: Schema) {
    const scriptConfig: CommandConfig = {
      id: existing ? existing.id : self.crypto.randomUUID(),
      enabled: values.enabled,
      name: values.name,
      command: values.command,
      aliases: [],
      outcome: values.outcome,
      cooldown: values.cooldown,
      require_role: values.require_role,
    };

    if (existing !== undefined) {
      // Update existing
      await $appDataMutation.mutateAsync({
        ...$appData,
        commands: $appData.commands.map((item) => {
          if (item.id !== existing.id) return item;
          return scriptConfig;
        }),
      });
    } else {
      // Add new
      await $appDataMutation.mutateAsync({
        ...$appData,
        commands: [...$appData.commands, scriptConfig],
      });
    }

    setIsDirty(false);
  }

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

  const outcomeTypeOptions = [
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

  function getOutcomeDefaults(type: CommandOutcomeType): OutcomeSchema {
    switch (type) {
      case CommandOutcomeType.Template:
        return {
          type: CommandOutcomeType.Template,
          message: "",
        };

      case CommandOutcomeType.Script:
        return {
          type: CommandOutcomeType.Script,
          script: exampleCode,
        };
    }
  }

  function onChangeOutcomeType(type: CommandOutcomeType) {
    const defaults = getOutcomeDefaults(type);
    setFields("outcome", defaults, true);
  }
</script>

<form use:form class="container">
  <div class="title-area">
    <div>
      <h1 class="title">{existing ? "Edit Command" : "Create Command"}</h1>
      <p class="text">
        {#if existing && $isDirty}
          Unsaved changes...
        {/if}
      </p>
    </div>
    <div class="actions">
      <button type="submit" class="btn">
        {existing ? "Save" : "Create"}
      </button>
      <a class="btn" href="/commands">Back</a>
    </div>
  </div>

  <div class="content">
    <Tabs.Root>
      <Tabs.List>
        <Tabs.Trigger value="settings"
          ><SolarSettingsBoldDuotone /> Settings</Tabs.Trigger
        >
        <Tabs.Trigger value="code"
          ><SolarCodeSquareBoldDuotone /> Code</Tabs.Trigger
        >
      </Tabs.List>
      <Tabs.Content value="code">
        {#if $data.outcome.type === CommandOutcomeType.Script}
          <section class="editor">
            <CodeEditor
              value={$data.outcome.script}
              onChange={(value) => {
                setFields("outcome.script", value, true);
                setIsDirty(true);
              }}
              onUserSave={() => {
                if (existing) save($data);
              }}
            />
          </section>
        {:else if $data.outcome.type === CommandOutcomeType.Template}
          <textarea
            id="outcome.script"
            name="outcome.script"
            style="width: 100%;height:100%"
          ></textarea>
        {/if}
      </Tabs.Content>
      <Tabs.Content value="settings">
        <div class="settings">
          <section class="section">
            <div class="section__head">
              <h2>Details</h2>
              <p>Basic details about the command</p>
            </div>

            <div class="row">
              <FormTextInput
                id="name"
                name="name"
                label="Name"
                description="Name for the command"
              />
              <FormTextInput
                id="command"
                name="command"
                label="Command"
                description="Message that will trigger this command"
              />
            </div>

            <FormCheckbox
              id="enabled"
              name="enabled"
              label="Enabled"
              checked={$data.enabled}
              onChecked={(checked) => {
                setFields("enabled", checked, true);
              }}
            />

            {#snippet outcomeTypeItem(item: (typeof outcomeTypeOptions)[0])}
              <div class="text-stack">
                <p class="text-stack--top">{item.label}</p>
                <p class="text-stack--bottom">{item.description}</p>
              </div>
            {/snippet}

            <FormSelect
              id="outcome.type"
              name="outcome.type"
              label="Command Type"
              items={outcomeTypeOptions}
              item={outcomeTypeItem}
              selected={$data.outcome.type}
              onChangeSelected={(selected) => {
                onChangeOutcomeType(selected);
              }}
            />
          </section>

          <!-- Cooldown and role requirements -->
          <section class="section">
            <div class="section__head">
              <h2>Cooldown, and requirements</h2>
              <p>
                Configure any cooldown, or requirements on this command trigger
              </p>
            </div>

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
              onChangeSelected={(selected) =>
                setFields("require_role", selected, true)}
            />

            <FormNumberInput id="cooldown" name="cooldown" label="Cooldown" />
          </section>
        </div>
      </Tabs.Content>
    </Tabs.Root>
  </div>
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

  .editor {
    position: relative;
    overflow: hidden;
    height: 100%;
  }

  .content {
    position: relative;
    flex: auto;
    overflow: hidden;
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
  }

  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
    height: 100%;
  }

  .title {
    color: #fff;
    margin-bottom: 0.25rem;
    line-height: 1;
    font-size: 1.75rem;
  }

  .text {
    color: #ccc;
  }

  .title-area {
    display: flex;
    align-items: center;
  }

  .actions {
    display: flex;
    flex: auto;
    justify-content: flex-end;
    gap: 1rem;
    align-items: center;
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

  .row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
</style>
