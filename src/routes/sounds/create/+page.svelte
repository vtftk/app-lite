<script lang="ts">
  import CreateSoundForm from "$lib/sections/sounds/CreateSoundForm.svelte";
</script>

<div class="container">
  <h1 class="title">Create Sound</h1>
  <p class="text">Create a sound that can be triggered</p>
  <a type="button" href="/sounds">Back</a>

  <CreateSoundForm />
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
