<script lang="ts">
  import { getAppData } from "$lib/api/runtimeAppData";

  import Events from "$lib/sections/events/Events.svelte";
  const appData = getAppData();
</script>

<div class="container">
  <h1 class="title">Events</h1>
  <p class="text">
    Setup specific triggers for events, such as throwing when a specific redeem
    is redeemed
  </p>

  <div class="grid">
    <Events events={$appData.events} />
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
  }

  .title {
    line-height: 1;
  }

  .grid {
    margin-top: 0.5rem;
  }
</style>
