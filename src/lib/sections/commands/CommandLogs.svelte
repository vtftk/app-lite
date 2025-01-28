<script lang="ts">
  import LogsTable from "$lib/sections/logs/LogsTable.svelte";
  import { getCommandLogs, bulkDeleteCommandLogs } from "$lib/api/commandModel";
  import {
    type LogId,
    type CommandId,
    type LogsQuery,
    type CommandLog,
  } from "$shared/dataV2";

  type Props = {
    id: CommandId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  async function onBulkDelete(logIds: LogId[]) {
    await bulkDeleteCommandLogs(id, logIds);
  }

  let logsPromise: Promise<CommandLog[]> | undefined = $state();

  function onRefresh() {
    logsPromise = getCommandLogs(id, query);
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
