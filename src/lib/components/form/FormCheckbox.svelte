<script lang="ts">
  import { Checkbox } from "bits-ui";
  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;
    label: string;
  };

  const { id, name, label }: Props = $props();
</script>

<div class="form-input">
  <label for={id}>{label}</label>

  <Checkbox.Root {id} {name} aria-labelledby="{name}-label">
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

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
</style>
