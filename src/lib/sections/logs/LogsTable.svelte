<script lang="ts">
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import { formatDate, formatTime } from "$lib/utils/date";
  import Button from "$lib/components/input/Button.svelte";
  import SolarRefreshBoldDuotone from "~icons/solar/refresh-bold-duotone";
  import { type LogId, type LogData, LoggingLevelStr } from "$shared/dataV2";
  import SolarInfoCircleBoldDuotone from "~icons/solar/info-circle-bold-duotone";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";
  import { confirmDialog } from "$lib/components/dialog/GlobalConfirmDialog.svelte";
  import SolarDangerTriangleBoldDuotone from "~icons/solar/danger-triangle-bold-duotone";

  type Props = {
    logs: LogData[];

    onRefresh: VoidFunction;
    onBulkDelete: (logs: LogId[]) => Promise<void>;
  };

  const { logs, onRefresh, onBulkDelete: _onBulkDelete }: Props = $props();

  let selected: LogId[] = $state([]);

  function onToggleSelected(item: LogId) {
    if (selected.includes(item)) {
      selected = selected.filter((id) => id !== item);
    } else {
      selected = [...selected, item];
    }
  }

  function onToggleAllSelected() {
    if (logs.length > 0 && selected.length === logs.length) {
      selected = [];
    } else {
      selected = logs.map((item) => item.id);
    }
  }

  async function onBulkDelete() {
    const confirm = await confirmDialog({
      title: "Confirm Delete",
      description: "Are you sure you want to delete the selected log entries?",
    });

    if (!confirm) {
      return;
    }

    const deletePromise = _onBulkDelete(selected);

    toast.promise(deletePromise, {
      loading: "Deleting log entries...",
      success: "Deleted log entries",
      error: toastErrorMessage("Failed to delete log entries"),
    });

    // Clear selection since all items are removed
    selected = [];
  }
</script>

<div class="container">
  <div class="selection">
    <div class="selection__count">
      {#if selected.length > 0}
        {selected.length} Selected
      {/if}
    </div>

    <div class="selection__actions">
      {#if selected.length > 0}
        <Button type="button" onclick={onBulkDelete}>
          <DeleteIcon /> Delete
        </Button>
      {/if}

      <Button type="button" onclick={onRefresh}>
        <SolarRefreshBoldDuotone /> Refresh
      </Button>
    </div>
  </div>

  <div class="wrapper">
    <table>
      <thead>
        <tr>
          <th class="column--select">
            <div class="select-actions">
              <ControlledCheckbox
                checked={logs.length > 0 && selected.length === logs.length}
                onCheckedChange={() => onToggleAllSelected()}
              />
            </div>
          </th>
          <th class="column--level">Level</th>
          <th class="column--msg">Message</th>
          <th class="column--date">Timestamp</th>
        </tr>
      </thead>
      <tbody>
        {#each logs as log}
          <tr>
            <td class="column--select">
              <div class="select-actions">
                <ControlledCheckbox
                  checked={selected.includes(log.id)}
                  onCheckedChange={() => onToggleSelected(log.id)}
                />
              </div>
            </td>
            <td class="column--level" data-level={log.level} title={log.level}>
              <span>
                {#if log.level == LoggingLevelStr.Debug}
                  <SolarInfoCircleBoldDuotone />
                {:else if log.level == LoggingLevelStr.Info}
                  <SolarInfoCircleBoldDuotone />
                {:else if log.level == LoggingLevelStr.Warn}
                  <SolarDangerTriangleBoldDuotone />
                {:else if log.level == LoggingLevelStr.Error}
                  <SolarDangerTriangleBoldDuotone />
                {/if}
              </span>
              <span class="log-level">{log.level}</span>
            </td>
            <td class="column--msg">
              <p class="message">{log.message}</p>
            </td>
            <td class="column--date">
              <span class="date-date">
                {formatDate(new Date(log.created_at))}
              </span>
              <span class="date-time">
                {formatTime(new Date(log.created_at))}
              </span>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .wrapper {
    width: 100%;
    overflow-x: hidden;
    position: relative;
    flex: auto;
  }

  .container {
    display: flex;
    flex-flow: column;
    width: 100%;
    height: 100%;
  }

  table {
    table-layout: fixed;
    width: 100%;
    max-width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    word-wrap: break-word;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0.5rem;
    border: 1px solid #333;
    border-top: none;
  }

  td {
    vertical-align: top;
  }

  thead {
    position: sticky;
    top: 0px;
    background-color: #222;
    z-index: 1;
  }

  table th {
    color: #fff;
    vertical-align: center;
  }

  .column--level {
    width: 4rem;
    text-align: center;
    color: #fff;
  }

  .column--level > span {
    margin: 0.5rem auto;
    margin-bottom: 0;
    font-size: 1.5rem;
    display: inline-block;
    line-height: 1;
  }

  .column--level > .log-level {
    font-size: 0.9rem;
    margin-top: 0rem;
  }

  .column--level[data-level="Info"] > span {
    color: #91d7ff;
  }

  .column--level[data-level="Debug"] > span {
    color: #c491ff;
  }

  .column--level[data-level="Warn"] > span {
    color: #ffc391;
  }

  .column--level[data-level="Error"] > span {
    color: #ff9191;
  }

  .column--msg {
    text-align: left;
  }

  .column--date {
    width: 7rem;
  }

  .date-date {
    color: #fff;
  }

  .date-time {
    color: #ccc;
    font-size: 0.9rem;
  }

  .message {
    position: relative;
    display: block;
    color: #ccc;
    overflow: hidden;
    font-family: "Jetbrains Mono";
    font-size: 0.9rem;
  }

  .column--select {
    width: 3.5rem;
  }

  .select-actions {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 0.5rem 0;
  }

  .selection {
    display: flex;
    align-items: center;
    gap: 1rem;
    height: 4rem;
    flex-shrink: 0;
    padding: 1rem;
    border: 1px solid #333;
  }

  .selection__count {
    flex: auto;
  }

  .selection__actions {
    display: flex;
    gap: 1rem;
  }
</style>
