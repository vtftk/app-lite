<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Button from "$lib/components/input/Button.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import {
    CalibrationStep,
    calibrationState as calibrationStep,
  } from "$lib/api/calibration";

  async function setCalibrationStep(step: CalibrationStep) {
    await invoke("set_calibration_step", { step });
  }
</script>

<div class="container">
  {#if $calibrationStep == CalibrationStep.NotStarted}
    <h1>Calibration not started</h1>
    <p>
      Press the <b>"Start"</b> button to begin calibrating the current model
    </p>
    <div class="row">
      <Button onclick={() => setCalibrationStep(CalibrationStep.Smallest)}>
        Start
      </Button>
      <LinkButton href="/">Cancel</LinkButton>
    </div>
  {:else if $calibrationStep == CalibrationStep.Smallest}
    <h1>Smallest size</h1>

    <p>Your model has been shrunk to its smallest size.</p>

    <p>
      Position the guide marker on your models head then press <b>"Done"</b>.
      Move your model until it fits within the window
      <b>DO NOT RESIZE YOUR MODEL</b>
    </p>

    <div>
      <img
        alt="OBS Interact"
        src="/help/guide-marker-head.jpg"
        height="128px"
        width="auto"
      />
    </div>

    <p>
      To move the guide marker press "Interact" in OBS while the Overlay is
      selected
    </p>

    <div>
      <img alt="OBS Interact" src="/help/obs-interact.jpg" />
    </div>

    <Button onclick={() => setCalibrationStep(CalibrationStep.Largest)}>
      Done
    </Button>
  {:else if $calibrationStep == CalibrationStep.Largest}
    <h1>Largest size</h1>
    <p>Your model has been grown to its largest.</p>

    <p>
      Position the guide marker on your models head then press <b>"Done"</b>. If
      your models head is out of view move your model into view

      <b>DO NOT RESIZE YOUR MODEL</b>
    </p>

    <div>
      <img
        alt="OBS Interact"
        src="/help/guide-marker-head.jpg"
        height="128px"
        width="auto"
      />
    </div>

    <p>
      To move the guide marker press "Interact" in OBS while the Overlay is
      selected
    </p>

    <div>
      <img alt="OBS Interact" src="/help/obs-interact.jpg" />
    </div>

    <Button onclick={() => setCalibrationStep(CalibrationStep.Complete)}>
      Done
    </Button>
  {:else if $calibrationStep == CalibrationStep.Complete}
    <h1>Complete</h1>
    <p>Calibration complete</p>
    <LinkButton
      onclick={() => setCalibrationStep(CalibrationStep.NotStarted)}
      href="/">Close</LinkButton
    >
  {/if}
</div>

<style>
  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    line-height: 1.5;
  }

  .container {
    padding: 1rem;
    display: flex;

    flex-flow: column;
    justify-content: flex-start;
    gap: 1rem;

    height: 100%;
    overflow: auto;
  }

  .row {
    display: flex;
    flex-flow: row;
    gap: 1rem;
  }
</style>
