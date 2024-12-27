<script lang="ts">
  import { onMount } from "svelte";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import { type LogId, type ScriptId, type LogsQuery } from "$shared/dataV2";
  import {
    eventLogsQuery,
    deleteEventLogs,
    invalidateEventLogs,
  } from "$lib/api/vevents";

  type Props = {
    id: ScriptId;
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
