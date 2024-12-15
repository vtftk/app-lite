<script lang="ts">
  import type { Snippet } from "svelte";
  import { scale } from "svelte/transition";
  import type { HTMLButtonAttributes } from "svelte/elements";
  import {
    Popover,
    type PopoverProps,
    type PopoverContentProps,
    type PopoverTriggerProps,
  } from "bits-ui";

  type Props = {
    children?: Snippet;
    content?: Snippet;

    rootProps?: PopoverProps;
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
  <Popover.Trigger {...triggerProps} class="btn" asChild let:builder>
    <button
      {...buttonProps}
      type="button"
      class="trigger btn"
      use:builder.action
      {...builder}
    >
      {@render children?.()}
    </button>
  </Popover.Trigger>
  <Popover.Content sideOffset={8} {...contentProps} asChild let:builder>
    <div
      transition:scale={{ duration: 200 }}
      class="content"
      use:builder.action
      {...builder}
    >
      {@render content?.()}
    </div>
  </Popover.Content>
</Popover.Root>

<style>
  .content {
    z-index: 30;
    width: 100%;
    max-width: 328px;
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
