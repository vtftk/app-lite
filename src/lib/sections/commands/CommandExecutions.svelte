<script lang="ts">
  import { onMount } from "svelte";
  import ExecutionsTable from "$lib/components/ExecutionsTable.svelte";
  import {
    type CommandId,
    type ExecutionId,
    type ExecutionsQuery,
  } from "$shared/dataV2";
  import {
    commandExecutionsQuery,
    deleteCommandExecutions,
    invalidateCommandExecutions,
  } from "$lib/api/commandModel";

  type Props = {
    id: CommandId;
  };

  const { id }: Props = $props();

  const query: ExecutionsQuery = $state({});

  const executionsQuery = $derived(commandExecutionsQuery(id, query));
  const executions = $derived($executionsQuery.data ?? []);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(executionIds: ExecutionId[]) {
    await deleteCommandExecutions(id, executionIds);
  }

  function onRefresh() {
    invalidateCommandExecutions(id, query);
  }
</script>

{#if $executionsQuery.isPending}
  Loading...
{/if}

<ExecutionsTable {onRefresh} {onBulkDelete} {executions} />
