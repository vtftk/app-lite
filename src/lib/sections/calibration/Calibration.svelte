<script lang="ts">
  import {
    CalibrationStep,
    calibrationState as calibrationStep,
  } from "$lib/api/calibration";
  import { invoke } from "@tauri-apps/api/core";

  async function setCalibrationStep(step: CalibrationStep) {
    await invoke("set_calibration_step", { step });
  }
</script>

{#if $calibrationStep == CalibrationStep.NotStarted}
  <h1>Calibration not started</h1>
  <p>Start the calibration</p>
  <button onclick={() => setCalibrationStep(CalibrationStep.Smallest)}>
    Start
  </button>
{:else if $calibrationStep == CalibrationStep.Smallest}
  <h1>Smallest size</h1>
  <p>
    Your model has been shrunk to its smallest, position the guide on your
    models head then press done. Click on the overlay to set the guide position.
    You can press "Interact" in OBS
  </p>
  <button onclick={() => setCalibrationStep(CalibrationStep.Largest)}>
    Done
  </button>
{:else if $calibrationStep == CalibrationStep.Largest}
  <h1>Largest size</h1>
  <p>
    Your model has been grown to its largest, position the guide on your models
    head then press done. Click on the overlay to set the guide position. You
    can press "Interact" in OBS
  </p>
  <button onclick={() => setCalibrationStep(CalibrationStep.Complete)}>
    Done
  </button>
{:else if $calibrationStep == CalibrationStep.Complete}
  <h1>Complete</h1>
  <p>Calibration complete</p>
  <button>Close</button>
{/if}
