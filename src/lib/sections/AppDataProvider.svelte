<script lang="ts">
  import { setContext } from "svelte";
  import type { RuntimeAppData } from "$lib/api/types";
  import { derived, type Readable } from "svelte/store";
  import {
    APP_DATA_CONTEXT,
    createAppDataQuery,
    RUNTIME_APP_DATA_CONTEXT,
    createRuntimeAppDataQuery,
  } from "$lib/api/runtimeAppData";

  const runtimeAppData = createRuntimeAppDataQuery();

  const runtimeAppDataStore: Readable<RuntimeAppData | undefined> = derived(
    runtimeAppData,
    ($runtimeAppData) => $runtimeAppData.data,
  );

  const appData = createAppDataQuery();
  const appDataStore: Readable<RuntimeAppData | undefined> = derived(
    appData,
    ($appData) => $appData.data,
  );

  setContext(RUNTIME_APP_DATA_CONTEXT, runtimeAppDataStore);
  setContext(APP_DATA_CONTEXT, appDataStore);
</script>

{#if $runtimeAppData.isLoading || $appData.isLoading}
  <p>Loading data...</p>
{:else if $runtimeAppDataStore !== undefined || $appDataStore !== undefined}
  <slot />
{/if}
