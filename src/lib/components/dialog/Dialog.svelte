<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { fade, scale } from "svelte/transition";
  import { Dialog, type WithoutChild } from "bits-ui";

  import Button from "../input/Button.svelte";

  type Props = Dialog.RootProps & {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;

    title?: Snippet;
    description?: Snippet;
    actions?: Snippet;
    contentProps?: WithoutChild<Dialog.ContentProps>;
    // ...other component props if you wish to pass them
  };

  let {
    open = $bindable(false),
    children,
    buttonLabel,
    button,
    contentProps,
    title,
    description,
    actions,
    ...restProps
  }: Props = $props();
</script>

<Dialog.Root bind:open {...restProps}>
  <Dialog.Trigger>
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
  </Dialog.Trigger>

  {#if open}
    <Dialog.Portal>
      <Dialog.Overlay>
        {#snippet child({ props })}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/snippet}
      </Dialog.Overlay>
      <Dialog.Content {...contentProps}>
        {#snippet child({ props })}
          <div {...props} class="content" transition:scale={{}}>
            {#if title}
              <Dialog.Title>
                {#snippet child({ props })}
                  <h3 {...props} class="title">{@render title()}</h3>
                {/snippet}
              </Dialog.Title>
            {/if}

            {#if description}
              <Dialog.Description>
                {#snippet child({ props })}
                  <p {...props} class="description">{@render description()}</p>
                {/snippet}
              </Dialog.Description>
            {/if}

            {@render children?.()}

            {#if actions}
              <div class="actions">
                {@render actions()}
              </div>
            {/if}
          </div>
        {/snippet}
      </Dialog.Content>
    </Dialog.Portal>
  {/if}
</Dialog.Root>

<style>
  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    z-index: 49;
  }

  .content {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);

    width: 100%;
    max-width: 40rem;

    background-color: #111;
    border: 1px solid #222;
    border-radius: 0.25rem;

    z-index: 50;
  }

  .title {
    background-color: #222;
    padding: 1rem;
    border-bottom: 1px solid #222;
    color: #999;
    font-size: 1rem;
  }

  .description {
    color: #ccc;
    padding: 1rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding: 1rem;
    gap: 1rem;
  }

  .actions :global(button) {
    padding: 0.5rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }
</style>
