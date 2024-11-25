<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { EventConfig } from "$lib/api/types";
  import EventForm from "$lib/sections/events/EventForm.svelte";
  import { derived, type Readable } from "svelte/store";

  const appData = getAppData();

  const item: Readable<EventConfig | undefined> = derived(
    [appData, page],
    ([$appData, $page]) => {
      const id = $page.params.id;
      const item = $appData.events.find((item) => item.id === id);
      return item;
    }
  );
</script>

{#if $item !== undefined}
  <div class="container">
    <h1 class="title">Edit Event</h1>
    <p class="text">Edit</p>
    <a type="button" href="/events">Back</a>

    <EventForm existing={$item} />
  </div>
{:else}
  <div class="container">
    <h1 class="title">Event Not Found</h1>
    <p class="text">Unknown event</p>
    <a type="button" href="/events">Back</a>
  </div>
{/if}

<style>
  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
    height: 100%;
    overflow: auto;
  }

  .title {
    line-height: 1;
  }
</style>
