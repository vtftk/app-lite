<script lang="ts">
  import { Checkbox, type CheckboxRootProps } from "bits-ui";

  type Props = {
    checked: boolean;
    onCheckedChange: VoidFunction;
  } & CheckboxRootProps;

  const { checked, onCheckedChange, ...props }: Props = $props();
</script>

<Checkbox.Root {...props} {checked} {onCheckedChange}>
  {#snippet children({ checked })}
    {#if checked}
      <span>&#10003;</span>
    {/if}
  {/snippet}
</Checkbox.Root>

<style>
  :global([data-checkbox-indicator]) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: #fff;
  }

  :global([data-checkbox-root]) {
    display: inline-flex;
    width: 32px;
    height: 32px;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    border: 1px solid #444;
    background-color: #333;
    transition: all 150ms ease-in-out;
    cursor: pointer;
    flex-shrink: 0;
  }

  :global([data-checkbox-root][data-state="checked"]) {
    background-color: #666;
    border: 1px solid #777;
  }

  :glboal([data-checkbox-root][data-state="unchecked"]:hover) {
    border: 1px solid #fff;
  }
</style>
