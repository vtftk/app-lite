<script lang="ts">
  import type { VTubeStudioBroadcast } from "$lib/api/types";

  import { detectVTubeStudio } from "$lib/api/data";
  import { getErrorMessage } from "$lib/utils/error";
  import SolarCardSearchBoldDuotone from "~icons/solar/card-search-bold-duotone";

  import Dialog from "./Dialog.svelte";
  import Button from "./input/Button.svelte";
  import DialogCloseButton from "./DialogCloseButton.svelte";

  type Props = {
    onChoosePort: (port: number) => void;
  };

  const { onChoosePort }: Props = $props();

  let promise: Promise<VTubeStudioBroadcast> | null = $state(null);

  function onScan() {
    promise = detectVTubeStudio();
  }
</script>

<Dialog
  buttonLabel={{
    icon: SolarCardSearchBoldDuotone,
    text: "Detect VTube Studio",
  }}
>
  <!-- Title -->
  {#snippet title()}
    Detect VTube Studio
  {/snippet}

  <!-- Content -->
  {#snippet children()}
    {#if promise}
      {#await promise}
        <p>Detecting locally running VTube Studio...</p>
      {:then r}
        <p>Found running VTube Studio</p>

        <p>Version: {r.apiVersion}</p>
        <p>Port: {r.data.port}</p>
      {:catch e}
        Failed to detect VTube Studio: {getErrorMessage(e)}
      {/await}
    {/if}
  {/snippet}

  <!-- Action buttons -->
  {#snippet actions()}
    {#if promise}
      {#await promise then r}
        <DialogCloseButton
          buttonLabel={{ text: `Choose ${r.data.port}` }}
          onclick={() => onChoosePort(r.data.port)}
        />
      {/await}
    {/if}
    <Button onclick={onScan}>Scan</Button>

    <DialogCloseButton buttonLabel={{ text: "Close" }} />
  {/snippet}
</Dialog>
