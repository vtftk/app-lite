<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";

  type Props = {
    name: string;
    id: string;
  };

  const { name, id }: Props = $props();

  const updateHotkeys = () => {
    invoke("update_hotkeys");
  };

  const runtimeAppData = getRuntimeAppData();
  onMount(() => {
    updateHotkeys();
  });
</script>

<button type="button" onclick={updateHotkeys}>Refresh Hotkeys</button>
<select {name} {id}>
  {#each $runtimeAppData.hotkeys as hotkey}
    <option value={hotkey.hotkey_id}>{hotkey.name}</option>
  {/each}
</select>
