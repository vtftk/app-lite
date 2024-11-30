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

<form use:form>
  <section class="section">
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

  <section class="section">
    <CodeEditor
      value={$data.script}
      onChange={(value) => {
        setFields("script", value, true);
      }}
    />
  </section>

  <button type="submit" class="btn">
    {existing ? "Save" : "Create"}
  </button>
</form>

<style>
  form {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }

  .section {
    display: flex;
    flex-flow: column;

    border: 1px solid #333;
    padding: 1rem;
    gap: 1.5rem;
  }
</style>
