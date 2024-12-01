<script lang="ts">
  import getBackendURL from "$lib/utils/url";
  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;
    label: string;
    scale?: number;

    // Existing source URL
    existing?: string;
  };

  const { id, name, label, existing, scale }: Props = $props();

  let inputElm: HTMLInputElement | undefined = $state();
  let currentImage = $state(existing);

  /**
   * Handle updates to the current image to
   * update the previews
   */
  function onChangeImage() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);

    if (file) {
      currentImage = URL.createObjectURL(file);
    } else {
      currentImage = undefined;
    }
  }
</script>

<div class="form-input">
  <label for={id}>{label}</label>

  <div class="image-preview-wrapper">
    {#if currentImage !== undefined}
      <img
        class="image-preview"
        src={getBackendURL(currentImage)}
        alt="Preview"
        style="transform: translate(-50%, -50%) scale({scale});"
      />
    {/if}
  </div>

  <button
    class="btn"
    type="button"
    onclick={() => {
      inputElm?.click();
    }}>{existing ? "Replace" : "Select"} Image</button
  >

  <input
    bind:this={inputElm}
    hidden
    style="display: none;"
    type="file"
    aria-describedby="{name}-validation"
    onchange={onChangeImage}
    accept="image/*"
    {id}
    {name}
  />

  <FormErrorLabel {name} />
</div>

<style>
  .image-preview-wrapper {
    position: relative;
    height: 300px;
    width: 300px;
    background-color: #000;
    margin: 1rem 0;
    overflow: hidden;
  }

  .image-preview {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
  }

  .form-input {
    display: inline-block;
    border-radius: 0.5rem;
  }

  .form-input label {
    font-size: 1rem;
  }
</style>
