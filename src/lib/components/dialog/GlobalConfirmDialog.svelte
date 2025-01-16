<script module lang="ts">
  type ConfirmMessage = {
    title: string;
    description: string;

    confirmLabel?: string;
    cancelLabel?: string;

    resolve: (value: boolean) => void;
  };

  let confirmStore = $state<ConfirmMessage[]>([]);

  export function confirmDialog(message: Omit<ConfirmMessage, "resolve">) {
    return new Promise<boolean>((resolve) => {
      confirmStore = [...confirmStore, { ...message, resolve }];
    });
  }
</script>

<script lang="ts">
  import Button from "$lib/components/input/Button.svelte";
  import Dialog from "$lib/components/dialog/Dialog.svelte";

  const currentMessage: ConfirmMessage | undefined = $derived(confirmStore[0]);

  function onResult(message: ConfirmMessage, value: boolean) {
    // Pop the message out of the store
    confirmStore = confirmStore.filter((_, index) => index !== 0);

    message.resolve(value);
  }
</script>

{#if currentMessage !== undefined}
  <Dialog open>
    {#snippet title()}{currentMessage.title}{/snippet}
    {#snippet description()}{currentMessage.description}{/snippet}
    {#snippet actions()}
      <Button
        type="button"
        onclick={() => {
          onResult(currentMessage, true);
        }}
      >
        {currentMessage.confirmLabel ?? "Confirm"}
      </Button>
      <Button
        type="button"
        onclick={() => {
          onResult(currentMessage, false);
        }}
      >
        {currentMessage.cancelLabel ?? "Cancel"}
      </Button>
    {/snippet}
  </Dialog>
{/if}
