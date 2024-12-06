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
  import {
    createAppDateMutation,
    createCreateCommandMutation,
    createUpdateCommandMutation,
    getAppData,
  } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import CodeEditor from "$lib/components/scripts/CodeEditor.svelte";
  import exampleCode from "../../../../script/example_command.js?raw";
  import { Tabs } from "bits-ui";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import SolarCodeSquareBoldDuotone from "~icons/solar/code-square-bold-duotone";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import FormSections from "$lib/components/form/FormSections.svelte";
  import FormSection from "$lib/components/form/FormSection.svelte";
  import FormBoundCheckbox from "$lib/components/form/FormBoundCheckbox.svelte";
  import RequiredRoleSelect from "../events/RequiredRoleSelect.svelte";
  import CommandOutcomeSelect from "./CommandOutcomeSelect.svelte";
  import { toast } from "svelte-sonner";

  type Props = {
    existing?: CommandConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const updateCommand = createUpdateCommandMutation(appData, appDataMutation);
  const createCommand = createCreateCommandMutation(appData, appDataMutation);

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
    outcome: getOutcomeDefaults(CommandOutcomeType.Script),
    require_role: MinimumRequiredRole.None,
    cooldown: 1000,
  };

  function createFromExisting(config: CommandConfig): Partial<Schema> {
    return {
      name: config.name,
      command: config.command,
      enabled: config.enabled,
      outcome: config.outcome,
      require_role: config.require_role,
      cooldown: config.cooldown,
    };
  }

  const { form, data, setFields, isDirty, setIsDirty } = createForm<
    z.infer<typeof schema>
  >({
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
            error: "Failed to save command",
          }
        : {
            loading: "Creating command...",
            success: "Created command",
            error: "Failed to create command",
          }
    );

    return savePromise;
  }

  async function save(values: Schema) {
    const partialCommandConfig: Omit<CommandConfig, "id"> = {
      enabled: values.enabled,
      name: values.name,
      command: values.command,
      aliases: [],
      outcome: values.outcome,
      cooldown: values.cooldown,
      require_role: values.require_role,
    };

    if (existing !== undefined) {
      await $updateCommand({
        commandId: existing.id,
        commandConfig: partialCommandConfig,
      });
    } else {
      const commandConfig: CommandConfig = {
        ...partialCommandConfig,
        id: self.crypto.randomUUID(),
      };

      await $createCommand({
        commandConfig,
      });
    }

    setIsDirty(false);
  }

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

<form use:form>
  {#snippet actions()}
    <button type="submit" class="btn">
      {existing ? "Save" : "Create"}
    </button>
    <a class="btn" href="/commands">Back</a>
  {/snippet}

  <PageLayoutList
    title={existing ? "Edit Command" : "Create Command"}
    description={existing && $isDirty ? "Unsaved changes..." : "..."}
    {actions}
  >
    <div class="content">
      <Tabs.Root>
        <Tabs.List>
          <Tabs.Trigger value="settings">
            <SolarSettingsBoldDuotone /> Settings
          </Tabs.Trigger>
          <Tabs.Trigger value="code">
            <SolarCodeSquareBoldDuotone /> Code
          </Tabs.Trigger>
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
                  if (existing) saveWithToast($data);
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
          <FormSections>
            <FormSection
              title="Details"
              description="Basic details about the command"
            >
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

              <FormBoundCheckbox
                id="enabled"
                name="enabled"
                label="Enabled"
                description="Whether this command can be used"
              />

              <CommandOutcomeSelect
                id="outcome.type"
                name="outcome.type"
                label="Command Type"
                selected={$data.outcome.type}
                onChangeSelected={(selected) => {
                  onChangeOutcomeType(selected);
                }}
              />
            </FormSection>

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
                onChangeSelected={(selected) =>
                  setFields("require_role", selected, true)}
              />

              <FormNumberInput id="cooldown" name="cooldown" label="Cooldown" />
            </FormSection>
          </FormSections>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  </PageLayoutList>
</form>

<style>
  .editor {
    position: relative;
    overflow: hidden;
    height: 100%;
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
  }
  .content :global([data-tabs-content]:nth-child(3)) {
    padding: 1rem;
  }

  form {
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
</style>
