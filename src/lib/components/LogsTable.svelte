<script lang="ts" generics="T extends LogData">
  import { formatDate, formatTime } from "$lib/utils/date";
  import { LoggingLevelStr, type LogData } from "$shared/dataV2";
  import SolarDangerTriangleBoldDuotone from "~icons/solar/danger-triangle-bold-duotone";
  import SolarInfoCircleBoldDuotone from "~icons/solar/info-circle-bold-duotone";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";

  type Props = {
    logs: T[];

    onDelete: (log: T) => void;
  };

  const { logs, onDelete }: Props = $props();
</script>

<div class="wrapper">
  <table>
    <thead>
      <tr>
        <th class="column--level">Level</th>
        <th class="column--msg">Message</th>
        <th class="column--date">Timestamp</th>
        <th class="column--actions">Action</th>
      </tr>
    </thead>
    <tbody>
      {#each logs as log}
        <tr>
          <td class="column--level" data-level={log.level}>
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
          <td class="column--actions">
            <div class="actions">
              <button class="btn" onclick={() => onDelete(log)}>
                <DeleteIcon />
              </button>
            </div>
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
    border: 1px solid #333;
  }

  table th {
    color: #fff;
  }

  .column--level {
    width: 4rem;
    text-align: center;
    color: #fff;
  }

  .column--level > span {
    margin: 0.5rem auto;
    font-size: 1.5rem;
    display: inline-block;
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
    /* height: 3rem; */
    overflow: hidden;
    font-family: "Jetbrains Mono";
    font-size: 0.9rem;
  }

  .column--actions {
    width: 4rem;
  }

  .column--actions .actions {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 0.5rem;
  }
</style>
