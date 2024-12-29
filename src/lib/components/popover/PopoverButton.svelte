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

  import Button from "../input/Button.svelte";

  type Props = {
    children?: Snippet;
    content?: Snippet;

    rootProps?: PopoverRootProps;
    triggerProps?: Omit<PopoverTriggerProps, "asChild">;
    contentProps?: Omit<PopoverContentProps, "asChild">;
  } & HTMLButtonAttributes;

  let open = $state(false);

  const {
    content,
    children,
    rootProps,
    triggerProps,
    contentProps,
    ...buttonProps
  }: Props = $props();
</script>

<Popover.Root controlledOpen {open} {...rootProps}>
  <Popover.Trigger {...triggerProps}>
    {#snippet child({ props })}
      <Button
        {...props}
        {...buttonProps}
        type="button"
        onclick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          open = !open;
        }}
      >
        {@render children?.()}
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Portal>
    <Popover.Content
      sideOffset={8}
      onInteractOutside={(event) => {
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        open = false;
      }}
      {...contentProps}
    >
      {#snippet child({ props, wrapperProps, open })}
        <div {...wrapperProps} class="wrapper">
          {#if open}
            <div
              {...props}
              transition:scale={{ duration: 200 }}
              class="content"
            >
              {@render content?.()}
            </div>
          {/if}
        </div>
      {/snippet}
    </Popover.Content>
  </Popover.Portal>
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
