<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import { derived } from "svelte/store";
  import FormSelect from "$lib/components/form/FormSelect.svelte";
  import SolarRefreshBold from "~icons/solar/refresh-bold";
  type Props = {
    id: string;
    name: string;
    label: string;

    selected: any;
    onChangeSelected: (value: any) => void;
  };

  const { id, label, name, selected, onChangeSelected }: Props = $props();

  const updateHotkeys = () => {
    invoke("update_hotkeys");
  };

  const runtimeAppData = getRuntimeAppData();

  onMount(() => {
    updateHotkeys();
  });

  const options = derived([runtimeAppData], ([$runtimeAppData]) =>
    $runtimeAppData.hotkeys.map((item) => ({
      value: item.hotkey_id,
      label: item.name,
    }))
  );
</script>

{#snippet hotkeyItem(item: (typeof $options)[0])}
  <div class="text-stack">
    <p class="text-stack--top">{item.label}</p>
  </div>
{/snippet}

<div class="container">
  <FormSelect
    {id}
    {name}
    {label}
    items={$options}
    item={hotkeyItem}
    {selected}
    {onChangeSelected}
  />

  <button type="button" class="btn" onclick={updateHotkeys}>
    <SolarRefreshBold /> Refresh Hotkeys
  </button>
</div>

{#if !$runtimeAppData.vtube_studio_connected}
  <p class="error">
    Not connected to VTube studio... Connect to see available Hotkeys
  </p>
{/if}

<style>
  .container {
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
    width: 100%;
  }

  .error {
    color: #dba33a;
  }
</style>
