<script lang="ts">
  import { getAppData } from "$lib/api/runtimeAppData";
  import Sounds from "$lib/sections/sounds/Sounds.svelte";

  const appData = getAppData();
</script>

<div class="container">
  <div class="title-area">
    <h1 class="title">Sounds</h1>
    <p class="text">Create sounds that can be triggered</p>
  </div>

  <div class="content-area">
    <div class="grid">
      <Sounds sounds={$appData.sounds} />
    </div>
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
    line-height: 1;
  }

  .grid {
    margin-top: 0.5rem;
  }

  .content-area {
    flex: auto;
    overflow-y: auto;
  }
</style>
