<script lang="ts">
  import { z } from "zod";
  import { createForm } from "felte";
  import { toast } from "svelte-sonner";
  import { goto } from "$app/navigation";
  import reporterDom from "@felte/reporter-dom";
  import { validator } from "@felte/validator-zod";
  import HTabs from "$lib/components/HTabs.svelte";
  import { toastErrorMessage } from "$lib/utils/error";
  import CardButton from "$lib/components/CardButton.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import { createCommand, updateCommand } from "$lib/api/commands";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import CodeEditor from "$lib/components/scripts/CodeEditor.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import MonacoEditor from "$lib/components/scripts/MonacoEditor.svelte";
  import SolarReorderBoldDuotone from "~icons/solar/reorder-bold-duotone";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import SolarCardSendBoldDuotone from "~icons/solar/card-send-bold-duotone";
  import FormBoundCheckbox from "$lib/components/form/FormBoundCheckbox.svelte";
  import SolarCodeSquareBoldDuotone from "~icons/solar/code-square-bold-duotone";
  import SolarChecklistMinimalisticBoldDuotone from "~icons/solar/checklist-minimalistic-bold-duotone";
  import {
    type Command,
    CommandOutcomeType,
    MinimumRequiredRole,
    MINIMUM_REQUIRED_ROLE_VALUES,
  } from "$lib/api/types";

  import CommandLogs from "./CommandLogs.svelte";
  import CommandExecutions from "./CommandExecutions.svelte";
  import exampleCode from "../../../../script/example_command.js?raw";
  import RequiredRoleSelect from "../events/RequiredRoleSelect.svelte";

  type Props = {
    existing?: Command;
  };

  const { existing }: Props = $props();

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

  const createDefaults: Schema = {
    name: "",
    command: "!test",
    enabled: true,
    outcome: getOutcomeDefaults(CommandOutcomeType.Template),
    require_role: MinimumRequiredRole.None,
    cooldown: 1000,
  };

  function createFromExisting(config: Command): Partial<Schema> {
    return {
      name: config.name,
      command: config.command,
      enabled: config.enabled,
      outcome: config.outcome,
      require_role: config.require_role,
      cooldown: config.cooldown,
    };
  }

  const { form, data, setFields, isDirty, setIsDirty } = createForm<Schema>({
    // Derive initial values
    initialValues: existing ? createFromExisting(existing) : createDefaults,

    // Validation and error reporting
    extend: [validator({ schema }), reporterDom()],

    onSubmit(values) {
      saveWithToast(values);

      if (!existing) {
        goto("/commands");
      }
    },
  });

  function saveWithToast(values: Schema) {
    const savePromise = save(values);

    toast.promise(
      savePromise,
      existing
        ? {
            loading: "Saving command...",
            success: "Saved command",
            error: toastErrorMessage("Failed to save command"),
          }
        : {
            loading: "Creating command...",
            success: "Created command",
            error: toastErrorMessage("Failed to create command"),
          },
    );

    return savePromise;
  }

  async function save(values: Schema) {
    const command = values.command.toLowerCase().trim();

    if (existing !== undefined) {
      await updateCommand({
        commandId: existing.id,
        update: {
          enabled: values.enabled,
          name: values.name,
          command,
          aliases: [],
          outcome: values.outcome,
          cooldown: values.cooldown,
          require_role: values.require_role,
        },
      });
    } else {
      await createCommand({
        enabled: values.enabled,
        name: values.name,
        command,
        aliases: [],
        outcome: values.outcome,
        cooldown: values.cooldown,
        require_role: values.require_role,
      });
    }

    setIsDirty(false);
  }

  function getOutcomeDefaults(type: CommandOutcomeType): OutcomeSchema {
    switch (type) {
      case CommandOutcomeType.Template:
        return {
          type: CommandOutcomeType.Template,
          message: "Hey $(user), this is the test command response",
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

  const commandTypeOption = [
    {
      icon: SolarCodeSquareBoldDuotone,
      color: "red",
      value: CommandOutcomeType.Template,
      label: "Template",
      description:
        "Create a simple text response with some basic templating. Simple commands with static responses",
    },
    {
      icon: SolarCodeSquareBoldDuotone,
      color: "purple",
      value: CommandOutcomeType.Script,
      label: "Script",
      description:
        "Create a command using scripting with JavaScript code. For powerful interactive messages",
    },
  ];
</script>

{#snippet settingsTabContent()}
  <FormSections>
    <FormSection title="Details" description="Basic details about the command">
      <FormTextInput
        id="name"
        name="name"
        label="Name"
        description="Name for the command"
        placeholder="Test Command"
      />
      <FormTextInput
        id="command"
        name="command"
        label="Command"
        description="Message that will trigger this command"
      />

      <FormBoundCheckbox
        id="enabled"
        name="enabled"
        label="Enabled"
        description="Whether this command can be used"
      />
    </FormSection>
  </FormSections>
{/snippet}

{#snippet typeTabContent()}
  <div class="event-trigger-grid">
    {#each commandTypeOption as option (option.value)}
      <CardButton
        icon={option.icon}
        color={option.color}
        label={option.label}
        description={option.description}
        selected={$data.outcome.type === option.value}
        onclick={() =>
          $data.outcome.type !== option.value &&
          onChangeOutcomeType(option.value)}
        contentVisible={$data.outcome.type === option.value}
      />
    {/each}
  </div>
{/snippet}

{#snippet codeTabContent()}
  {#if $data.outcome.type === CommandOutcomeType.Script}
    <section class="editor">
      <CodeEditor
        value={$data.outcome.script}
        onChange={(value) => {
          setFields("outcome.script", value, true);
          setIsDirty(true);
        }}
        onUserSave={() => {
          if (existing) saveWithToast($data);
        }}
      />
    </section>
  {:else if $data.outcome.type === CommandOutcomeType.Template}
    <div class="template-split">
      <section class="editor">
        <MonacoEditor
          language="commandTemplateFormat"
          value={$data.outcome.message}
          onChange={(value) => {
            setFields("outcome.message", value, true);
            setIsDirty(true);
          }}
          onUserSave={() => {
            if (existing) saveWithToast($data);
          }}
          options={{
            wordWrap: "on",
          }}
        />
      </section>

      <div class="hints">
        <p>
          If your response message is longer than 500 characters it will be
          split into multiple messages and sent separately
        </p>
        <p>Templating</p>
        <ul>
          <li>
            $(user) - Replaced with the name of the user using the command
          </li>
          <li>
            $(touser) - Replaced with the name of the user this command is
            targeting (First provided twitch username)
          </li>
        </ul>
      </div>
    </div>
  {/if}
{/snippet}

{#snippet requirementsTabContent()}
  <!-- Cooldown and role requirements -->
  <FormSection
    title="Cooldown, and requirements"
    description="Configure any cooldown, or requirements on this command trigger"
  >
    <RequiredRoleSelect
      id="require_role"
      name="require_role"
      label="Minimum Required Role"
      selected={$data.require_role}
      onChangeSelected={(selected) => setFields("require_role", selected, true)}
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
  </FormSection>
{/snippet}

{#snippet executionsTabContent()}
  {#if existing !== undefined}
    <CommandExecutions id={existing.id} />
  {/if}
{/snippet}

{#snippet logsTabContent()}
  {#if existing !== undefined}
    <CommandLogs id={existing.id} />
  {/if}
{/snippet}

<form use:form>
  {#snippet actions()}
    {#if existing && $isDirty}
      Unsaved changes...
    {/if}

    <button type="submit" class="btn">
      {existing ? "Save" : "Create"}
    </button>
    <a class="btn" href="/commands">Back</a>
  {/snippet}

  <PageLayoutList
    title={existing ? "Edit Command" : "Create Command"}
    description={existing
      ? `Editing "${existing.name}"`
      : "Create an event that will trigger some outcome"}
    {actions}
  >
    <HTabs
      tabs={[
        {
          value: "details",
          icon: SolarSettingsBoldDuotone,
          label: "Details",
          content: settingsTabContent,
        },
        {
          value: "type",
          icon: SolarCardSendBoldDuotone,
          label: "Type",
          content: typeTabContent,
        },

        {
          value: "code",
          icon: SolarCodeSquareBoldDuotone,
          label:
            $data.outcome.type === CommandOutcomeType.Template
              ? "Template"
              : "Code",
          content: codeTabContent,
          disablePadding: true,
        },
        {
          value: "requirements",
          icon: SolarChecklistMinimalisticBoldDuotone,
          label: "Requirements",
          content: requirementsTabContent,
        },
        ...(existing !== undefined
          ? [
              {
                value: "executions",
                icon: SolarReorderBoldDuotone,
                label: "Executions",
                content: executionsTabContent,
                disablePadding: true,
              },
              {
                value: "logs",
                icon: SolarReorderBoldDuotone,
                label: "Logs",
                content: logsTabContent,
                disablePadding: true,
              },
            ]
          : []),
      ]}
    />
  </PageLayoutList>
</form>

<style>
  .editor {
    position: relative;
    overflow: hidden;
    height: 100%;
  }

  .template-split {
    display: flex;
    flex-direction: row;
    height: 100%;
  }

  .template-split .editor {
    flex: auto;
  }

  form {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .hints {
    max-width: 14rem;
  }

  .event-trigger-grid {
    display: grid;

    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
</style>
