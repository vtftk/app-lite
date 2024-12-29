<script lang="ts">
  import type { RuntimeAppData } from "$lib/api/types";

  import { setContext } from "svelte";
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
  <div class="skeleton-list">
    <div class="skeleton" style="width: 90%; height: 1.5rem;"></div>
    <div class="skeleton" style="width: 70%; height: 1rem;"></div>
    <div class="skeleton" style="width: 80%; height: 1rem;"></div>
  </div>
{:else if $runtimeAppDataStore !== undefined || $appDataStore !== undefined}
  <slot />
{/if}
