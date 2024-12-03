<script lang="ts">
  import { getAppData } from "$lib/api/runtimeAppData";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import EventItem from "$lib/sections/events/EventItem.svelte";

  const appData = getAppData();
</script>

{#snippet actions()}
  <a class="btn" href="/events/create"> Create Event </a>
{/snippet}

<PageLayoutList
  title="Events"
  description="Setup specific triggers for events, such as throwing when a specific redeem is redeemed"
  {actions}
>
  <div class="grid">
    {#each $appData.events as event}
      <EventItem config={event} />
    {/each}
  </div>
</PageLayoutList>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }
</style>
