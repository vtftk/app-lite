<script lang="ts">
  import EventForm from "$lib/sections/events/EventForm.svelte";
</script>

<div class="container">
  <div class="title-area">
    <div>
      <h1 class="title">Create Event</h1>
      <p class="text">Create an event that will trigger some outcome</p>
    </div>
    <div class="actions">
      <a type="button" class="btn" href="/events">Back</a>
    </div>
  </div>
  <div class="content-area">
    <EventForm />
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
    height: 100%;
  }

  .title {
    color: #fff;
    margin-bottom: 0.25rem;
    line-height: 1;
    font-size: 1.75rem;
  }

  .text {
    color: #ccc;
  }

  .title-area {
    display: flex;
  }

  .actions {
    display: flex;
    flex: auto;
    justify-content: flex-end;
    gap: 1rem;
    align-items: center;
  }

  .content-area {
    flex: auto;
    overflow-y: auto;
  }
</style>
