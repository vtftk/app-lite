<script lang="ts">
  import MonacoEditor from "./MonacoEditor.svelte";

  type Props = {
    value: string;
    onChange: (value: string) => void;

    onUserSave?: VoidFunction;
  };

  const { value, onChange, onUserSave }: Props = $props();
</script>

<MonacoEditor language="javascript" {value} {onChange} {onUserSave} />
