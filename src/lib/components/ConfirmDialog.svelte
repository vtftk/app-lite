<script lang="ts">
  import Dialog from "./Dialog.svelte";
  import Button from "./input/Button.svelte";

  type Props = {
    open?: boolean;

    title: string;
    description: string;

    confirmLabel?: string;
    cancelLabel?: string;

    onconfirm?: VoidFunction;
    oncancel?: VoidFunction;
  };

  let {
    open = $bindable(false),
    title: dialogTitle,
    description: dialogDescription,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    onconfirm,
    oncancel,
  }: Props = $props();
</script>

<Dialog bind:open>
  {#snippet title()}{dialogTitle}{/snippet}
  {#snippet description()}{dialogDescription}{/snippet}
  {#snippet actions()}
    <Button
      type="button"
      onclick={() => {
        onconfirm?.();
        open = false;
      }}
    >
      {confirmLabel}
    </Button>
    <Button
      type="button"
      onclick={() => {
        oncancel?.();
        open = false;
      }}
    >
      {cancelLabel}
    </Button>
  {/snippet}
</Dialog>
