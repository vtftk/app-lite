<script lang="ts">
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
</script>

<div class="container">
  <h1 class="title">Create Throwable</h1>
  <p class="text">Create a new item that can be thrown</p>
  <a type="button" href="/throwables">Back</a>

  <ThrowableForm />
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
