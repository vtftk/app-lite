<script lang="ts">
  import { Dialog, Separator } from "bits-ui";
  import { fade, scale } from "svelte/transition";

  import MonacoEditor from "./scripts/MonacoEditor.svelte";

  type Props = {
    metadata: unknown;
  };

  const { metadata }: Props = $props();

  let open = $state(false);

  const metadataValue = $derived(JSON.stringify(metadata, undefined, 2));
</script>

<Dialog.Root
  {open}
  onOpenChange={(value) => {
    open = value;
  }}
>
  <Dialog.Trigger type="button">View Metadata</Dialog.Trigger>
  {#if open}
    <Dialog.Portal>
      <Dialog.Overlay transition={fade} transitionConfig={{ duration: 150 }} />
      <Dialog.Content transition={scale} class="metadata-dialog-content">
        <Dialog.Title>Metadata</Dialog.Title>

        <Separator.Root />

        <section class="editor">
          <MonacoEditor
            language="json"
            value={metadataValue}
            readOnly
            onChange={() => {}}
          />
        </section>
        <div data-dialog-actions>
          <Dialog.Close>
            <span class="sr-only">Close</span>
          </Dialog.Close>
        </div>
      </Dialog.Content>
    </Dialog.Portal>
  {/if}
</Dialog.Root>

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
