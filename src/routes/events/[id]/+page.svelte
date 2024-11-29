<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { EventConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
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
  {#snippet actions()}
    <a type="button" class="btn" href="/events">Back</a>
  {/snippet}

  <PageLayoutList
    title="Create Event"
    description="Create an event that will trigger some outcome"
    {actions}
  >
    <EventForm existing={$item} />
  </PageLayoutList>
{:else}
  {#snippet actions()}
    <a type="button" class="btn" href="/events">Back</a>
  {/snippet}

  <PageLayoutList title="Event Not Found" description="Unknown event" {actions}>
    <EventForm existing={$item} />
  </PageLayoutList>
{/if}
