<script lang="ts">
  import SoundForm from "$lib/sections/sounds/SoundForm.svelte";
</script>

<div class="container">
  <h1 class="title">Create Sound</h1>
  <p class="text">Create a sound that can be triggered</p>
  <a type="button" href="/sounds">Back</a>

  <SoundForm />
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
