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
    console.log($runtimeAppData);
    invoke("update_hotkeys");
  };

  const runtimeAppData = getRuntimeAppData();
  console.log($runtimeAppData);
  onMount(() => {
    updateHotkeys();
  });
</script>

<button onclick={updateHotkeys}>Refresh Hotkeys</button>
<select {name} {id}>
  {#each $runtimeAppData.hotkeys as hotkey}
    <option value={hotkey.hotkey_id}>{hotkey.name}</option>
  {/each}
</select>
