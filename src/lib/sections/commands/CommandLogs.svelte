<script lang="ts">
  import { onMount } from "svelte";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import { type LogId, type CommandId, type LogsQuery } from "$shared/dataV2";
  import {
    commandLogsQuery,
    invalidateCommandLogs,
    bulkDeleteCommandLogsMutation,
  } from "$lib/api/commands";

  type Props = {
    id: CommandId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(commandLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  const bulkDeleteCommandLogs = bulkDeleteCommandLogsMutation(id);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(logIds: LogId[]) {
    await $bulkDeleteCommandLogs.mutateAsync({
      logIds,
    });
  }

  function onRefresh() {
    invalidateCommandLogs(id, query);
  }
</script>

{#if $logsQuery.isPending}
  Loading...
{/if}

<LogsTable {onRefresh} {onBulkDelete} {logs} />
