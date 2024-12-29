<script lang="ts">
  import { createField } from "felte";
  import { Slider as BitsSlider, type WithoutChildren } from "bits-ui";

  import Slider from "../input/Slider.svelte";
  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;
    value: number;
    showTicks?: boolean;
  } & Omit<WithoutChildren<BitsSlider.RootProps>, "value" | "onValueChange">;

  let {
    id,
    name,
    label,
    description,
    value,
    ref = $bindable(null),
    min,
    max,
    step,
    ...restProps
  }: Props = $props();

  const { field, onInput } = createField(name);
</script>

<div class="form-input">
  <label for={id}>{label}</label>
  {#if description}
    <p class="description">{description}</p>
  {/if}

  <div class="row">
    <div class="wrapper">
      <Slider
        value={[value]}
        bind:ref
        {...restProps}
        onValueChange={(value) => {
          onInput(value[0]);
        }}
        {min}
        {max}
        {step}
      />
    </div>

    <input
      use:field
      data-felte-keep-on-remove
      type="number"
      {id}
      {name}
      {min}
      {max}
      {step}
      {value}
      aria-describedby="{name}-validation"
    />
  </div>

  <FormErrorLabel {name} />
</div>

<style>
  .wrapper {
    flex: auto;
    width: 100%;
  }

  .row {
    display: flex;
    width: 100%;
    align-items: center;
    flex-flow: row;
    gap: 1rem;
  }

  .form-input {
    display: inline-flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .form-input label {
    font-size: 1rem;
    color: #fff;
  }

  .form-input input {
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .description {
    font-size: 0.8rem;
    color: #999;
  }
</style>
