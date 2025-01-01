<script lang="ts">
  import { onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Aside from "$lib/components/Aside.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import {
    CalibrationStep,
    calibrationState as calibrationStep,
  } from "$lib/api/calibration";

  let resetOnDestroy = true;

  async function setCalibrationStep(step: CalibrationStep) {
    await invoke("set_calibration_step", { step });
  }

  function onReset() {
    resetOnDestroy = false;
    setCalibrationStep(CalibrationStep.NotStarted);
  }

  // Reset calibration state on component destroy if its not already been reset
  onDestroy(() => {
    if (!resetOnDestroy) {
      onReset();
    }
  });

  let currentStep: number = $derived.by(() => {
    if ($calibrationStep === CalibrationStep.NotStarted) {
      return 1;
    }
    if ($calibrationStep === CalibrationStep.Smallest) {
      return 2;
    }
    if ($calibrationStep === CalibrationStep.Largest) {
      return 3;
    }
    if ($calibrationStep === CalibrationStep.Complete) {
      return 4;
    }

    return 1;
  });
</script>

{#snippet step({ step, label }: { step: number; label: string })}
  <div class="step" class:step--complete={currentStep >= step}>
    <span class="step__line"></span>
    <span class="step__number">{step}</span>
    <span class="step__label">{label}</span>
  </div>
{/snippet}

<PageLayoutList
  title="Calibration"
  description="Calibrate your model for throwing items"
>
  <div class="container">
    <div class="steps">
      {@render step({ step: 1, label: "Begin Calibration" })}
      {@render step({ step: 2, label: "Smallest Size" })}
      {@render step({ step: 3, label: "Largest Size" })}
      {@render step({ step: 4, label: "Complete" })}
    </div>

    <div class="content">
      {#if $calibrationStep == CalibrationStep.NotStarted}
        <Aside title="IMPORTANT" severity="error">
          During the calibration process your model will shrink and grow.
          <br />
          <br />
          <b>DO NOT</b> manually resize your model at any point. The size set
          during the calibration process is important for calibration will
          ensure items at thrown at your model correctly
          <br />
          <br />
          If you accidentally resize your model press the "Cancel" button and start
          again
        </Aside>

        <p>
          Press the <b>"Start"</b> button to begin calibrating the current model
        </p>
      {:else if $calibrationStep == CalibrationStep.Smallest}
        <div class="row">
          <p>
            Your model has been shrunk to its smallest size.
            <br /><br />
            Position the guide marker on your models head then press
            <b>"Done" </b>
            <br /><br />
            If your model is out of view, move your model vertically so that it is
            within view then place the guide marker on your models head.
          </p>

          <div style="flex-shrink: 0;">
            <img
              alt="OBS Interact"
              src="/help/guide-marker-head.jpg"
              height="128px"
              width="auto"
            />
          </div>
        </div>

        <Aside severity="tip">
          To move the guide marker press "Interact" in OBS while the Overlay is
          selected.
          <div>
            <img alt="OBS Interact" src="/help/obs-interact.jpg" />
          </div>
        </Aside>

        <Aside severity="warning">
          <b>DO NOT RESIZE YOUR MODEL</b>. The sizing is important for
          calibration. If you've accidentally resized your model press "Cancel"
          and start again
        </Aside>
      {:else if $calibrationStep == CalibrationStep.Largest}
        <div class="row">
          <p>
            Your model has grown to its largest size. Position the guide marker
            on your models head then press <b>"Done"</b>
          </p>

          <div style="flex-shrink: 0;">
            <img
              alt="OBS Interact"
              src="/help/guide-marker-head.jpg"
              height="128px"
              width="auto"
            />
          </div>
        </div>

        <Aside severity="tip">
          To move the guide marker press "Interact" in OBS while the Overlay is
          selected

          <div>
            <img alt="OBS Interact" src="/help/obs-interact.jpg" />
          </div>
        </Aside>

        <Aside severity="warning">
          <b>DO NOT RESIZE YOUR MODEL</b>. The sizing is important for
          calibration. If you've accidentally resized your model press "Cancel"
          and start again
        </Aside>
      {:else if $calibrationStep == CalibrationStep.Complete}
        <Aside severity="success"
          >Calibration complete, you can now throw items at your model. Press
          the "Close" button to return to the Home tab or press any other tab on
          the sidebar</Aside
        >
      {/if}
    </div>

    <div class="actions">
      {#if $calibrationStep == CalibrationStep.NotStarted}
        <LinkButton href="/" onclick={onReset}>Cancel</LinkButton>

        <Button onclick={() => setCalibrationStep(CalibrationStep.Smallest)}>
          Start
        </Button>
      {:else if $calibrationStep == CalibrationStep.Smallest}
        <LinkButton href="/" onclick={onReset}>Cancel</LinkButton>

        <Button onclick={() => setCalibrationStep(CalibrationStep.Largest)}>
          Done
        </Button>
      {:else if $calibrationStep == CalibrationStep.Largest}
        <LinkButton href="/" onclick={onReset}>Cancel</LinkButton>

        <Button onclick={() => setCalibrationStep(CalibrationStep.Complete)}>
          Done
        </Button>
      {:else if $calibrationStep == CalibrationStep.Complete}
        <LinkButton onclick={onReset} href="/">Close</LinkButton>
      {/if}
    </div>
  </div>
</PageLayoutList>

<style>
  .steps {
    display: flex;
    margin-bottom: 1rem;
  }

  .step {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    position: relative;
  }

  .step--complete .step__line {
    background-color: #368a36;
  }

  .step--complete .step__label {
    color: #6ed86e;
  }

  .step--complete .step__number {
    background-color: #368a36;
    color: #fff;
    font-weight: bold;
  }

  .step--current .step__line {
    background: linear-gradient(90deg, #6ed86e 70%, #ffffff);
  }

  .step:not(:last-of-type) {
    flex: auto;
  }

  .step__line {
    position: absolute;
    height: 3px;
    background-color: #444;
    width: 100%;
    left: 0;
    top: 50%;
    z-index: -1;
    transform: translateY(-50%);
  }

  .step__number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    background-color: #444;
    border-radius: 100%;
  }

  .step__label {
    background-color: #111;
    padding: 0.5rem;
  }

  p {
    color: #ccc;
    line-height: 1.5;
  }

  .container {
    flex: auto;
    display: flex;

    flex-flow: column;
    justify-content: flex-start;

    overflow: hidden;
    position: relative;
    height: 100%;
  }

  .content {
    display: flex;
    flex-flow: column;
    gap: 1rem;
    overflow: auto;
    flex: auto;
  }

  .actions {
    justify-content: flex-end;
    display: flex;
    flex-flow: row;
    gap: 1rem;
  }

  .actions :global(.btn) {
    min-width: 8rem;
    justify-content: center;
  }

  .row {
    display: flex;
    gap: 1rem;
    width: 100%;
    justify-content: space-between;
  }
</style>
