<script lang="ts">
  import { invalidateScriptLogs, scriptLogsQuery } from "$lib/api/scripts";
  import LogsTable from "$lib/components/LogsTable.svelte";
  import {
    type LogsQuery,
    type ScriptId,
    type ScriptLog,
  } from "$shared/dataV2";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  type Props = {
    id: ScriptId;
  };

  const { id }: Props = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(scriptLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  onMount(() => {
    invalidateScriptLogs(id, query);
  });

  async function onDelete(log: ScriptLog) {
    if (!confirm("Are you sure you want to delete this log entry?")) {
      return;
    }

    const deletePromise = async () => {};

    toast.promise(deletePromise, {
      loading: "Deleting log entry...",
      success: "Deleted log entry",
      error: "Failed to delete log entry",
    });
  }
</script>

<LogsTable {onDelete} {logs} />
