<script lang="ts">
  import { derived } from "svelte/store";
  import { createModelDataQuery } from "$lib/api/calibration";
  import { createIsAuthenticatedQuery } from "$lib/api/oauth";
  import {
    getRuntimeAppData,
    createDeriveModelCalibrated,
  } from "$lib/api/runtimeAppData";

  const runtimeAppData = getRuntimeAppData();
  const isAuthenticated = createIsAuthenticatedQuery();

  const modelDataQuery = createModelDataQuery();
  const modelData = derived(
    modelDataQuery,
    ($modelDataQuery) => $modelDataQuery.data ?? [],
  );

  // Model needs to be calibrated if not available here
  const isModelCalibrated = createDeriveModelCalibrated(
    modelData,
    runtimeAppData,
  );
</script>

<div class="status">
  <div class="status-item">
    <div>VTube Studio</div>
    <div
      class="status-indicator"
      data-status={$runtimeAppData.vtube_studio_connected
        ? $isModelCalibrated && $runtimeAppData.vtube_studio_auth
          ? "green"
          : "orange"
        : "red"}
    ></div>
  </div>
  <div class="status-item">
    <div>Active Overlay</div>
    <div
      class="status-indicator"
      data-status={$runtimeAppData.active_overlay_count > 0 ? "green" : "red"}
    >
      {$runtimeAppData.active_overlay_count}
    </div>
  </div>
  <div class="status-item">
    <div>Twitch</div>
    <div
      class="status-indicator"
      data-status={$isAuthenticated.data ? "green" : "red"}
    ></div>
  </div>
</div>

<style>
  .status {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    margin: 0.5rem 0;
  }

  .status-item {
    display: flex;
    border: 1px solid #333;
    justify-content: space-between;
    padding: 0.5rem;
    align-items: center;
    border-radius: 0.5rem;
  }

  .status-indicator {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    color: #fff;
    width: 2rem;
    height: 2rem;
    background-color: black;
    border-radius: 1rem;
  }

  .status-indicator[data-status="green"] {
    background-color: green;
  }

  .status-indicator[data-status="orange"] {
    background-color: orange;
  }

  .status-indicator[data-status="red"] {
    background-color: red;
  }
</style>
