<script lang="ts">
  import { onMount } from "svelte";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import { type LogId, type ScriptId, type LogsQuery } from "$shared/dataV2";
  import {
    scriptLogsQuery,
    invalidateScriptLogs,
    bulkDeleteScriptLogsMutation,
  } from "$lib/api/scripts";

  type Props = {
    id: ScriptId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(scriptLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  const bulkDeleteScriptLogs = bulkDeleteScriptLogsMutation(id);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(logIds: LogId[]) {
    await $bulkDeleteScriptLogs.mutateAsync({
      logIds,
    });
  }

  function onRefresh() {
    invalidateScriptLogs(id, query);
  }
</script>

{#if $logsQuery.isPending}
  Loading...
{/if}

<LogsTable {onRefresh} {onBulkDelete} {logs} />
