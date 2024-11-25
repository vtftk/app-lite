<script lang="ts">
  import EventForm from "$lib/sections/events/EventForm.svelte";
</script>

<div class="container">
  <h1 class="title">Create Event</h1>
  <p class="text">Create an event that will trigger some outcome</p>
  <a type="button" href="/events">Back</a>

  <EventForm />
</div>

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
