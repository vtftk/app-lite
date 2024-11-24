<script lang="ts">
  import type { EventConfig } from "$lib/api/types";
  import EventItem from "./EventItem.svelte";

  type Props = {
    events: EventConfig[];
  };

  const { events }: Props = $props();
</script>

<div class="grid">
  <a class="create" href="/events/create"> Create Event </a>

  {#each events as event}
    <EventItem config={event} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  .create {
    background-color: #222;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;

    padding: 1rem;

    cursor: pointer;
    border: none;
    color: #fff;
    align-items: center;
    justify-content: center;
    font-size: 1em;
  }
</style>
