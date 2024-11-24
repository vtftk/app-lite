<script lang="ts">
  import CreateEventForm from "$lib/sections/events/CreateEventForm.svelte";
</script>

<div class="container">
  <h1 class="title">Create Event</h1>
  <p class="text">Create an event that will trigger some outcome</p>
  <a type="button" href="/events">Back</a>

  <CreateEventForm />
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
</style>
