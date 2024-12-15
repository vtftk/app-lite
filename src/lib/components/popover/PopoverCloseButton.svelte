<script lang="ts">
  import { Popover, type PopoverCloseProps } from "bits-ui";

  type Props = {
    children?: Snippet;

    closeProps?: Omit<PopoverCloseProps, "asChild">;
  } & HTMLButtonAttributes;

  const { children, closeProps, ...buttonProps }: Props = $props();
</script>

<Popover.Close {...closeProps} class="btn" asChild let:builder>
  <button
    {...buttonProps}
    type="button"
    class="trigger btn"
    use:builder.action
    {...builder}
  >
    {@render children?.()}
  </button>
</Popover.Close>
