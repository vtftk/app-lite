<script lang="ts">
  import { createForm } from "felte";
  import { validator } from "@felte/validator-zod";
  import reporterDom from "@felte/reporter-dom";
  import { z } from "zod";
  import type { SoundConfig, UserScriptConfig } from "$lib/api/types";
  import { invoke } from "@tauri-apps/api/core";
  import { createAppDateMutation, getAppData } from "$lib/api/runtimeAppData";
  import { goto } from "$app/navigation";
  import FormErrorLabel from "$lib/components/form/FormErrorLabel.svelte";
  import SoundUpload from "$lib/components/form/SoundUpload.svelte";
  import FormTextInput from "$lib/components/form/FormTextInput.svelte";
  import FormNumberInput from "$lib/components/form/FormNumberInput.svelte";
  import CodeEditor from "$lib/components/scripts/CodeEditor.svelte";
  import exampleCode from "../../../../script/example.js?raw";
  import FormCheckbox from "$lib/components/form/FormCheckbox.svelte";

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

  const { form, data, setFields } = createForm<z.infer<typeof schema>>({
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

      goto("/scripts");
    },
  });
</script>

<form use:form class="container">
  <div class="title-area">
    <div>
      <h1 class="title">Edit Script</h1>
      <p class="text">Editing Script</p>
    </div>
    <div class="actions">
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

      <button type="submit" class="btn">
        {existing ? "Save" : "Create"}
      </button>
      <a class="btn" href="/scripts">Back</a>
    </div>
  </div>

  <section class="editor">
    <CodeEditor
      value={$data.script}
      onChange={(value) => {
        setFields("script", value, true);
      }}
    />
  </section>
</form>

<style>
  .editor {
    position: relative;
    flex: auto;
    overflow: hidden;
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
  }

  .actions {
    display: flex;
    flex: auto;
    justify-content: flex-end;
    gap: 1rem;
    align-items: center;
  }
</style>
