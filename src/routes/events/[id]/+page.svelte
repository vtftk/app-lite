<script lang="ts">
  import { page } from "$app/stores";
  import { createEventQuery } from "$lib/api/vevents";
  import EventForm from "$lib/sections/events/EventForm.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";

  const id = $derived($page.params.id);
  const eventQuery = $derived(createEventQuery(id));
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
