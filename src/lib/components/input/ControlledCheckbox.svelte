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
