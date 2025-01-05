<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { Dialog, type DialogCloseProps } from "bits-ui";

  import Button from "./input/Button.svelte";

  type Props = {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;
  } & DialogCloseProps;

  const { button, buttonLabel, ...buttonProps }: Props = $props();
</script>

<Dialog.Close {...buttonProps}>
  {#snippet child({ props })}
    {#if button}
      {@render button({ props })}
    {:else if buttonLabel}
      <Button {...props} type="button">
        {#if buttonLabel.icon}
          <buttonLabel.icon />
        {/if}

        {buttonLabel.text}
      </Button>
    {/if}
  {/snippet}
</Dialog.Close>
