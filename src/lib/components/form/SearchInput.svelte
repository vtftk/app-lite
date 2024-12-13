<script lang="ts">
  type Props = {
    value: string;
    placeholder?: string;

    type?: "text" | "password";
  };

  let { value = $bindable(), placeholder, type = "text" }: Props = $props();
</script>

<div class="form-input">
  <input {type} bind:value {placeholder} aria-describedby="{name}-validation" />
</div>

<style>
  .form-input {
    display: inline-flex;
    flex-flow: column;
    gap: 0.5rem;
    flex: auto;
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
</style>
