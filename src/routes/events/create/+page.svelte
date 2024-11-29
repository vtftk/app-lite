<script lang="ts">
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import EventForm from "$lib/sections/events/EventForm.svelte";
</script>

{#snippet actions()}
  <a type="button" class="btn" href="/events">Back</a>
{/snippet}

<PageLayoutList
  title="Create Event"
  description="Create an event that will trigger some outcome"
  {actions}
>
  <EventForm />
</PageLayoutList>
