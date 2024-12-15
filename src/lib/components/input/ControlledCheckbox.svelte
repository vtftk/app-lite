<script lang="ts">
  import { Checkbox, type CheckboxProps } from "bits-ui";

  type Props = {
    checked: boolean;
    onCheckedChange: VoidFunction;
  } & CheckboxProps;

  const { checked, onCheckedChange, ...props }: Props = $props();
</script>

<Checkbox.Root {...props} {checked} {onCheckedChange}>
  <Checkbox.Indicator let:isChecked>
    {#if isChecked}
      <span>&#10003;</span>
    {/if}
  </Checkbox.Indicator>
</Checkbox.Root>
