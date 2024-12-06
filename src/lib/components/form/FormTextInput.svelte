<script lang="ts">
  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    id: string;
    name: string;
    label: string;
    description?: string;

    type?: "text" | "password";
  };

  const { id, name, label, description, type = "text" }: Props = $props();
</script>

<div class="form-input">
  <label for={id}>{label}</label>
  <input {type} {id} {name} aria-describedby="{name}-validation" />
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
