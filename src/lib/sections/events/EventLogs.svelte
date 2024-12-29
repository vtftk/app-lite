<script lang="ts">
  import type { LogId, EventId, LogsQuery } from "$shared/dataV2";

  import { onMount } from "svelte";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import {
    eventLogsQuery,
    deleteEventLogs,
    invalidateEventLogs,
  } from "$lib/api/vevents";

  type Props = {
    id: EventId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(eventLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(logIds: LogId[]) {
    await deleteEventLogs(id, logIds);
  }

  function onRefresh() {
    invalidateEventLogs(id, query);
  }
</script>

{#if $logsQuery.isPending}
  Loading...
{/if}

<LogsTable {onRefresh} {onBulkDelete} {logs} />
