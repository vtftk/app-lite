<script lang="ts">
  import Dialog from "./Dialog.svelte";
  import MonacoEditor from "./scripts/MonacoEditor.svelte";
  import DialogCloseButton from "./DialogCloseButton.svelte";

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
      <MonacoEditor
        language="json"
        value={metadataValue}
        readOnly
        onChange={() => {}}
      />
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
</style>
