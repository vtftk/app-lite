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
</style>
