<script lang="ts">
  import "$lib/api/events";
  import Calibration from "$lib/sections/calibration/Calibration.svelte";
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import { derived } from "svelte/store";
  import Throwables from "$lib/sections/throwables/Throwables.svelte";

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
</script>

<main class="container">
  <p>Connected to VTube Studio: {$runtimeAppData.vtube_studio_connected}</p>
  <p>Current Model ID: {$runtimeAppData.model_id}</p>
  <p>Model Calibrated: {$isModelCalibrated}</p>

  {#if !isModelCalibrated}
    <Calibration />
  {/if}

  <Throwables items={$appData.items} />
</main>
