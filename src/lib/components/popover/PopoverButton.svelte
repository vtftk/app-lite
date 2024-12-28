<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  import { scale } from "svelte/transition";
  import {
    Popover,
    type PopoverRootProps,
    type PopoverContentProps,
    type PopoverTriggerProps,
  } from "bits-ui";

  type Props = {
    children?: Snippet;
    content?: Snippet;

    rootProps?: PopoverRootProps;
    triggerProps?: Omit<PopoverTriggerProps, "asChild">;
    contentProps?: Omit<PopoverContentProps, "asChild">;
  } & HTMLButtonAttributes;

  const {
    content,
    children,
    rootProps,
    triggerProps,
    contentProps,
    ...buttonProps
  }: Props = $props();
</script>

<Popover.Root {...rootProps}>
  <Popover.Trigger {...triggerProps}>
    {#snippet child({ props })}
      <button {...props} {...buttonProps} type="button" class="trigger btn">
        {@render children?.()}
      </button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content sideOffset={8} {...contentProps}>
    {#snippet child({ props, wrapperProps, open })}
      <div {...wrapperProps}>
        {#if open}
          <div {...props} transition:scale={{ duration: 200 }} class="content">
            {@render content?.()}
          </div>
        {/if}
      </div>
    {/snippet}
  </Popover.Content>
</Popover.Root>

<style>
  .content {
    z-index: 30;
    min-width: 12rem;
    border-radius: 12px;
    border: 1px solid #333;
    padding: 0.5rem;
    background-color: #222;
    box-shadow: 4px 0 10px #000;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;
  }
</style>
