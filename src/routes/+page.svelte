<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "$lib/api/events";
  import Calibration from "$lib/sections/Calibration.svelte";
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import { derived } from "svelte/store";
  import Throwables from "$lib/sections/Throwables.svelte";
  import type { ThrowableConfig } from "$lib/api/types";

  const appData = getAppData();
  const runtimeAppData = getRuntimeAppData();

  // Model needs to be calibrated if not available here
  const isModelCalibrated = derived(
    [appData, runtimeAppData],
    ([$appData, $runtimeAppData]) => {
      // No model active
      if ($runtimeAppData.model_id === null) {
        return false;
      }

      const modelData = $appData.models[$runtimeAppData.model_id];
      return modelData !== undefined;
    }
  );

  const testData: ThrowableConfig[] = [
    {
      name: "Heart",
      image: {
        pixelate: false,
        scale: 0.5,
        src: "https://clipartcraft.com/images/transparent-hearts-tiny-3.png",
        weight: 1,
      },
      sound: null,
    },
  ];
</script>

<main class="container">
  <p>Connected to VTube Studio: {$runtimeAppData.vtube_studio_connected}</p>
  <p>Current Model ID: {$runtimeAppData.model_id}</p>
  <p>Model Calibrated: {$isModelCalibrated}</p>

  {#if !isModelCalibrated}
    <Calibration />
  {/if}

  <Throwables items={[...testData, ...testData, ...testData]} />
</main>
