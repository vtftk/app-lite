<script lang="ts">
  import type { LogId, EventId, EventLog, LogsQuery } from "$shared/dataV2";

  import LogsTable from "$lib/sections/logs/LogsTable.svelte";
  import { getEventLogs, deleteEventLogs } from "$lib/api/eventModel";

  type Props = {
    id: EventId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  async function onBulkDelete(logIds: LogId[]) {
    await deleteEventLogs(id, logIds);
  }

  let logsPromise: Promise<EventLog[]> | undefined = $state();

  function onRefresh() {
    logsPromise = getEventLogs(id, query);
  }

  $effect(() => {
    onRefresh();
  });
</script>

{#if logsPromise}
  {#await logsPromise}
    <div
      class="skeleton"
      style="width: 90%; height: 1.5rem; padding: 1rem"
    ></div>
  {:then logs}
    <LogsTable {onRefresh} {onBulkDelete} {logs} />
  {/await}
{/if}
