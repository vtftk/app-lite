<script lang="ts">
  import { onMount } from "svelte";
  import ExecutionsTable from "$lib/components/ExecutionsTable.svelte";
  import {
    type CommandId,
    type ExecutionId,
    type ExecutionsQuery,
  } from "$shared/dataV2";
  import {
    invalidateCommandLogs,
    commandExecutionsQuery,
    deleteCommandExecutionsMutation,
  } from "$lib/api/commands";

  type Props = {
    id: CommandId;
  };

  const { id }: Props = $props();

  const query: ExecutionsQuery = $state({});

  const executionsQuery = $derived(commandExecutionsQuery(id, query));
  const executions = $derived($executionsQuery.data ?? []);

  const deleteCommandExecutions = deleteCommandExecutionsMutation(id);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(executionIds: ExecutionId[]) {
    await $deleteCommandExecutions.mutateAsync({
      executionIds,
    });
  }

  function onRefresh() {
    invalidateCommandLogs(id, query);
  }
</script>

{#if $executionsQuery.isPending}
  Loading...
{/if}

<ExecutionsTable {onRefresh} {onBulkDelete} {executions} />
