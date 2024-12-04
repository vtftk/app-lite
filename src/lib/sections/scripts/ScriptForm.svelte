<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import type { UserScriptConfig } from "$lib/api/types";
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

  type Props = {
    existing?: UserScriptConfig;
  };

  const { existing }: Props = $props();

  const appData = getAppData();
  const appDataMutation = createAppDateMutation();

  const schema = z.object({
    name: z.string().min(1, "You must specify a name"),
    enabled: z.boolean(),
    script: z.string(),
  });

  type Schema = z.infer<typeof schema>;

  const { form, data, setFields, isDirty, setIsDirty } = createForm<
    z.infer<typeof schema>
  >({
    initialValues: existing
      ? {
          name: existing.name,
          enabled: existing.enabled,
          script: existing.script,
        }
      : {
          name: "",
          enabled: true,
          script: exampleCode,
        },

    extend: [validator({ schema }), reporterDom()],
    async onSubmit(values, context) {
      await save(values);
      goto("/scripts");
    },
  });

  async function save(values: Schema) {
    // Determine what events the script handles
    const events = await invoke<string[]>("test_get_script_events", {
      script: values.script,
    });

    const scriptConfig: UserScriptConfig = {
      id: existing ? existing.id : self.crypto.randomUUID(),
      enabled: values.enabled,
      name: values.name,
      script: values.script,
      events,
    };

    if (existing !== undefined) {
      // Update existing
      await $appDataMutation.mutateAsync({
        ...$appData,
        scripts: $appData.scripts.map((item) => {
          if (item.id !== existing.id) return item;
          return scriptConfig;
        }),
      });
    } else {
      // Add new
      await $appDataMutation.mutateAsync({
        ...$appData,
        scripts: [...$appData.scripts, scriptConfig],
      });
    }

    setIsDirty(false);
  }
</script>

<form use:form class="container">
  <div class="title-area">
    <div>
      <h1 class="title">{existing ? "Edit Script" : "Create Script"}</h1>
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
      <a class="btn" href="/scripts">Back</a>
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
        <section class="editor">
          <CodeEditor
            value={$data.script}
            onChange={(value) => {
              setFields("script", value, true);
              setIsDirty(true);
            }}
            onUserSave={() => {
              if (existing) save($data);
            }}
          />
        </section>
      </Tabs.Content>
      <Tabs.Content value="settings">
        <div class="settings">
          <section class="section">
            <div class="section__head">
              <h2>Details</h2>
              <p>Basic details about the script</p>
            </div>
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
</style>
