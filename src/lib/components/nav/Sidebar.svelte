<script lang="ts">
  import Navigation from "./Navigation.svelte";
  import StatusDetails from "./StatusDetails.svelte";
</script>

<div class="sidebar">
  <Navigation />
  <StatusDetails />
</div>

<style>
  .sidebar {
    padding: 0.5rem;
    width: 16rem;
    flex-shrink: 0;
    background-color: #151515;
    border-right: 1px solid #222;
    z-index: 1;
  }
</style>
