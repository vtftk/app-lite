<script lang="ts">
  import { page } from "$app/stores";
  import { createEventQuery } from "$lib/api/vevents";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import EventForm from "$lib/sections/events/EventForm.svelte";
  import { derived } from "svelte/store";

  const id = derived(page, ($page) => $page.params.id);
  const eventQuery = createEventQuery(id);
</script>

{#if $eventQuery.isLoading}
  Loading...
{:else if $eventQuery.data}
  <EventForm existing={$eventQuery.data} />
{:else}
  {#snippet actions()}
    <a type="button" class="btn" href="/events">Back</a>
  {/snippet}

  <PageLayoutList title="Event Not Found" description="Unknown event" {actions}
  ></PageLayoutList>
{/if}
