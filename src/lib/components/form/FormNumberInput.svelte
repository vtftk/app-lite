<script lang="ts">
  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    min?: number;
    max?: number;
    step?: number;
  };

  const { id, name, label, description, min, max, step }: Props = $props();
</script>

<div class="form-input">
  <label for={id}>{label}</label>
  <input
    type="number"
    {id}
    {name}
    {min}
    {max}
    {step}
    aria-describedby="{name}-validation"
  />
  {#if description}
    <p class="description">{description}</p>
  {/if}
  <FormErrorLabel {name} />
</div>

<style>
  .form-input {
    display: inline-flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .form-input label {
    font-size: 1rem;
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
