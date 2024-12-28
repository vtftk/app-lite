<script lang="ts">
  import { Checkbox } from "bits-ui";

  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;

    label: string;
    description?: string;

    checked: boolean;
    onChecked: (checked: boolean) => void;
  };

  const { id, name, label, description, checked, onChecked }: Props = $props();
</script>

<div class="form-input">
  <Checkbox.Root
    {id}
    {name}
    aria-labelledby="{name}-label"
    {checked}
    onCheckedChange={(checked) => {
      onChecked(checked === true);
    }}
  >
    {#snippet children({ checked })}
      {#if checked}
        <span>&#10003;</span>
      {/if}
    {/snippet}
  </Checkbox.Root>

  <div>
    <label for={id}>{label}</label>

    {#if description}
      <p class="description">{description}</p>
    {/if}

    <FormErrorLabel {name} />
  </div>
</div>

<style>
  .form-input {
    display: inline-flex;
    flex-flow: row;
    gap: 0.75rem;
    align-items: center;
  }

  .form-input label {
    font-size: 1rem;
    color: #fff;
    margin-bottom: 0.25rem;
    display: block;
  }

  .description {
    font-size: 0.9rem;
    color: #ccc;
  }
</style>
