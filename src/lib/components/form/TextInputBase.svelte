<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  import FormErrorLabel from "./FormErrorLabel.svelte";

  type Props = {
    name: string;
    description?: string;
    placeholder?: string;

    type?: "text" | "password";
  } & Omit<HTMLInputAttributes, "name" | "placeholder" | "id" | "type">;

  const {
    name,
    placeholder,
    description,
    type = "text",
    ...props
  }: Props = $props();
</script>

<div class="form-input">
  <input
    {...props}
    data-felte-keep-on-remove
    {type}
    {name}
    {placeholder}
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
