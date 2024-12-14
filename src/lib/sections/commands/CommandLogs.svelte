<script lang="ts">
  import {
    commandLogsQuery,
    invalidateCommandLogs,
    type GetCommandLogs,
  } from "$lib/api/commands";
  import { formatDate, formatTime } from "$lib/utils/date";
  import {
    LoggingLevelStr,
    type CommandId,
    type LogsQuery,
  } from "$shared/dataV2";
  import { onMount } from "svelte";
  import SolarDangerTriangleBoldDuotone from "~icons/solar/danger-triangle-bold-duotone";
  import SolarInfoCircleBoldDuotone from "~icons/solar/info-circle-bold-duotone";

  type Props = {
    id: CommandId;
  };

  const { id } = $props();

  const query: LogsQuery = $state({});

  const logsQuery = $derived(commandLogsQuery(id, query));
  const logs = $derived($logsQuery.data ?? []);

  onMount(() => {
    invalidateCommandLogs(id, query);
  });
</script>

<div class="wrapper">
  <table>
    <thead>
      <tr>
        <th class="column--level">Level</th>
        <th class="column--msg">Message</th>
        <th class="column--date">Timestamp</th>
      </tr>
    </thead>
    <tbody>
      {#each logs as log}
        <tr>
          <td class="column--level">
            {#if log.level == LoggingLevelStr.Debug}
              <SolarInfoCircleBoldDuotone />
            {:else if log.level == LoggingLevelStr.Info}
              <SolarInfoCircleBoldDuotone />
            {:else if log.level == LoggingLevelStr.Warn}
              <SolarDangerTriangleBoldDuotone />
            {:else if log.level == LoggingLevelStr.Error}
              <SolarDangerTriangleBoldDuotone />
            {/if}
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

<style>
  .wrapper {
    width: 100%;
    height: 100%;
    max-width: 100%;
    overflow-x: hidden;
    position: relative;
    padding: 1rem;
  }

  table {
    table-layout: fixed;
    width: 100%;
    max-width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    word-wrap: break-word; /* Forces text to break inside cells */
    overflow: hidden; /* Prevents text from spilling out */
    text-overflow: ellipsis; /* Optional: Adds "..." for clipped text */
    padding: 0.5rem;
    vertical-align: top;
    border: 1px solid #888;
  }

  table th {
    color: #fff;
  }

  .column--level {
    width: 4rem;
    text-align: left;
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
    /* height: 3rem; */
    overflow: hidden;
    font-family: "Jetbrains Mono";
    font-size: 0.9rem;
  }
</style>
