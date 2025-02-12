<script lang="ts">
  import Dialog from "$lib/components/dialog/Dialog.svelte";
  import DialogCloseButton from "$lib/components/dialog/DialogCloseButton.svelte";

  type Props = {
    metadata: unknown;
  };

  const { metadata }: Props = $props();

  const metadataValue = $derived(JSON.stringify(metadata, undefined, 2));
</script>

<Dialog
  buttonLabel={{ text: "View Metadata" }}
  contentProps={{ class: "metadata-dialog-content" }}
>
  <!-- Title -->
  {#snippet title()}Metadata{/snippet}

  <!-- Content -->
  {#snippet children()}
    <section class="editor">
      <textarea class="metadata" name="" id="" readonly value={metadataValue}
      ></textarea>
    </section>
  {/snippet}

  <!-- Action buttons -->
  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: "Close" }} />
  {/snippet}
</Dialog>

<style>
  .editor {
    position: relative;
    overflow: hidden;
    height: 70vh;
    max-height: 30rem;
  }

  :global(.metadata-dialog-content) {
    width: 60vw;
    max-width: 60rem;
  }

  .metadata {
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #ccc;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    width: 100%;
    height: 100%;
    resize: none;
  }
</style>
