<script lang="ts">
  import type { Snippet, Component } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  import { Dialog } from "bits-ui";

  type Props = {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;
  } & HTMLButtonAttributes;

  const { button, buttonLabel }: Props = $props();
</script>

<Dialog.Close>
  {#snippet child({ props })}
    {#if button}
      {@render button({ props })}
    {:else if buttonLabel}
      <button {...props} class="btn" type="button">
        {#if buttonLabel.icon}
          <buttonLabel.icon />
        {/if}

        {buttonLabel.text}</button
      >
    {/if}
  {/snippet}
</Dialog.Close>
