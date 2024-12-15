<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

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
    class="trigger btn"
    use:builder.action
    {...builder}
    type="button"
  >
    {@render children?.()}
  </button>
</Popover.Close>
