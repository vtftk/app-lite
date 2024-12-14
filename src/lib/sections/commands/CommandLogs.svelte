<script lang="ts">
  import {
    bulkDeleteCommandLogsMutation,
    commandLogsQuery,
    invalidateCommandLogs,
  } from "$lib/api/commands";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import { type CommandId, type LogId, type LogsQuery } from "$shared/dataV2";
  import { onMount } from "svelte";

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
