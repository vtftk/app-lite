<script lang="ts">
  import { getAppData } from "$lib/api/runtimeAppData";

  import Throwables from "$lib/sections/throwables/Throwables.svelte";

  const appData = getAppData();
</script>

<div class="container">
  <div class="title-area">
    <h1 class="title">Throwables</h1>
    <p class="text">Items that can be thrown. Configure them below</p>
  </div>

  <div class="content-area">
    <div class="grid">
      <Throwables items={$appData.items} />
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
