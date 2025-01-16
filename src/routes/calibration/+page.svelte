<script lang="ts">
  import { getAppContext } from "$lib/api/runtimeAppData";
  import Calibration from "$lib/sections/calibration/Calibration.svelte";

  const appContext = getAppContext();
  const runtimeAppData = $derived(appContext.runtimeAppData);
</script>

{#if runtimeAppData.vtube_studio_connected}
  <Calibration />
{:else}
  <div>
    <h1>Not Connected</h1>
    <p>
      VTube studio is not connected, please ensure you have setup the overlay in
      OBS (and have OBS running) and have approved access in VTube studio
    </p>
  </div>
{/if}
