<script lang="ts">
  import type { Snippet } from "svelte";
  import { Popover, type PopoverCloseProps } from "bits-ui";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    children?: Snippet;

    closeProps?: Omit<PopoverCloseProps, "asChild">;
  } & HTMLButtonAttributes;

  const { children, closeProps, ...buttonProps }: Props = $props();
</script>

<Popover.Close {...closeProps} class="btn" asChild let:builder>
  <button
    {...buttonProps}
    class="trigger btn"
    use:builder.action
    {...builder}
    type="button"
  >
    {@render children?.()}
  </button>
</Popover.Close>
