<script lang="ts">
  import type { VTubeStudioBroadcast } from "$lib/api/types";

  import { Dialog, Separator } from "bits-ui";
  import { fade, scale } from "svelte/transition";
  import { detectVTubeStudio } from "$lib/api/data";
  import { getErrorMessage } from "$lib/utils/error";

  import Button from "./input/Button.svelte";

  type Props = {
    onChoosePort: (port: number) => void;
  };

  const { onChoosePort }: Props = $props();

  let promise: Promise<VTubeStudioBroadcast> | null = $state(null);

  function onScan() {
    promise = detectVTubeStudio();
  }
</script>

<Dialog.Root>
  <Dialog.Trigger type="button">Detect VTube Studio</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay transition={fade} transitionConfig={{ duration: 150 }} />
    <Dialog.Content transition={scale}>
      <Dialog.Title>Detect VTube Studio</Dialog.Title>

      <Separator.Root />

      {#if promise}
        {#await promise}
          <p>Detecting locally running VTube Studio...</p>
        {:then r}
          <p>Found running VTube Studio</p>

          <p>Version: {r.apiVersion}</p>
          <p>Version: {r.data.port}</p>
        {:catch e}
          Failed to detect VTube Studio: {getErrorMessage(e)}
        {/await}
      {/if}

      <div data-dialog-actions>
        {#if promise}
          {#await promise then r}
            <Dialog.Close onclick={() => onChoosePort(r.data.port)}>
              <span class="sr-only">Choose {r.data.port}</span>
            </Dialog.Close>
          {/await}
        {/if}
        <Button onclick={onScan}>Scan</Button>
        <Dialog.Close>
          <span class="sr-only">Close</span>
        </Dialog.Close>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
