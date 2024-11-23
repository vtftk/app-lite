<script lang="ts">
  import {
    APP_DATA_CONTEXT,
    createAppDataQuery,
    createRuntimeAppDataQuery,
    RUNTIME_APP_DATA_CONTEXT,
  } from "$lib/api/runtimeAppData";
  import type { RuntimeAppData } from "$lib/api/types";
  import { setContext } from "svelte";
  import { derived, type Readable } from "svelte/store";

  const runtimeAppData = createRuntimeAppDataQuery();

  const runtimeAppDataStore: Readable<RuntimeAppData | undefined> = derived(
    runtimeAppData,
    ($runtimeAppData) => $runtimeAppData.data
  );

  const appData = createAppDataQuery();
  const appDataStore: Readable<RuntimeAppData | undefined> = derived(
    appData,
    ($appData) => $appData.data
  );

  setContext(RUNTIME_APP_DATA_CONTEXT, runtimeAppDataStore);
  setContext(APP_DATA_CONTEXT, appDataStore);
</script>

{#if $runtimeAppData.isLoading || $appData.isLoading}
  <p>Loading data...</p>
{:else if $runtimeAppDataStore !== undefined || $appDataStore !== undefined}
  <slot />
{/if}
