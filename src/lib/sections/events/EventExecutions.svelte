<script lang="ts">
  import { onMount } from "svelte";
  import ExecutionsTable from "$lib/sections/executions/ExecutionsTable.svelte";
  import {
    type EventId,
    type ExecutionId,
    type ExecutionsQuery,
  } from "$shared/dataV2";
  import {
    eventExecutionsQuery,
    deleteEventExecutions,
    invalidateEventExecutions,
  } from "$lib/api/eventModel";

  type Props = {
    id: EventId;
  };

  const { id }: Props = $props();

  const query: ExecutionsQuery = $state({});

  const executionsQuery = $derived(eventExecutionsQuery(id, query));
  const executions = $derived($executionsQuery.data ?? []);

  onMount(() => {
    onRefresh();
  });

  async function onBulkDelete(executionIds: ExecutionId[]) {
    await deleteEventExecutions(id, executionIds);
  }

  function onRefresh() {
    invalidateEventExecutions(id, query);
  }
</script>

{#if $executionsQuery.isPending}
  Loading...
{/if}

<ExecutionsTable {onRefresh} {onBulkDelete} {executions} />
