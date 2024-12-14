<script lang="ts">
  import { invalidateScriptLogs, scriptLogsQuery } from "$lib/api/scripts";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import { type LogId, type LogsQuery, type ScriptId } from "$shared/dataV2";
  import { onMount } from "svelte";

  type Props = {
    id: ScriptId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(scriptLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(logs: LogId[]) {}

  function onRefresh() {
    invalidateScriptLogs(id, query);
  }
</script>

{#if $logsQuery.isPending}
  Loading...
{/if}

<LogsTable {onRefresh} {onBulkDelete} {logs} />
