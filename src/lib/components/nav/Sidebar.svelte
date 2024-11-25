<script lang="ts">
  import Navigation from "./Navigation.svelte";
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import { derived } from "svelte/store";

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

<div class="sidebar">
  <Navigation />

  <p>Connected to VTube Studio: {$runtimeAppData.vtube_studio_connected}</p>
  <p>Current Model ID: {$runtimeAppData.model_id}</p>
  <p>Model Calibrated: {$isModelCalibrated}</p>
  <p>Active Overlays: {$runtimeAppData.active_overlay_count}</p>

  <a href="/calibration">Begin Calibration</a>
</div>

<style>
  .sidebar {
    padding: 0.5rem;
    width: 18rem;
    flex-shrink: 0;
    background-color: #151515;
    border-right: 1px solid #222;
  }
</style>
