<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api";

  type Props = {
    value: string;
    onChange: (value: string) => void;

    onUserSave?: VoidFunction;
  };

  const { value, onChange, onUserSave }: Props = $props();

  let editor: Monaco.editor.IStandaloneCodeEditor | undefined;
  let monaco: typeof Monaco | undefined;
  let editorContainer: HTMLElement | undefined = $state();

  onMount(async () => {
    monaco = (await import("$lib/monaco")).default;

    // Your monaco instance is ready, let's display some code!
    editor = monaco.editor.create(editorContainer!, {
      theme: "vs-dark",
      automaticLayout: true,
      language: "typescript",
      tabSize: 4,
      detectIndentation: false,
    });

    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
      if (onUserSave) onUserSave();
    });

    const model = monaco.editor.createModel(
      value,
      "typescript",
      monaco.Uri.parse("file:///main.ts")
    );
    editor.setModel(model);

    editor.onDidChangeModelContent((event) => {
      const newValue = model.getValue();
      onChange(newValue);
    });
  });

  onDestroy(() => {
    monaco?.editor.getModels().forEach((model) => model.dispose());
    editor?.dispose();
    editor = undefined;
    monaco = undefined;
  });

  // Change editor value when value changes
  $effect(() => {
    editor?.setValue(value);
  });
</script>

<svelte:window
  onresize={() => {
    if (!editor) return;
    // make editor as small as possible
    editor.layout({ width: 0, height: 0 });

    // wait for next frame to ensure last layout finished
    window.requestAnimationFrame(() => {
      if (!editorContainer || !editor) return;
      const rect = editorContainer.getBoundingClientRect();
      editor.layout({ width: rect.width, height: rect.height });
    });
  }}
/>

<div class="container" bind:this={editorContainer}></div>

<style>
  .container {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
</style>
