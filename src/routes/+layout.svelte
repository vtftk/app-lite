<script lang="ts">
  // Apply global styling
  import "$lib/styles/global.scss";
  // Font family for code editor
  import "@fontsource/jetbrains-mono";
  import { navigating } from "$app/stores";
  import { queryClient } from "$lib/api/utils";
  import Sidebar from "$lib/components/nav/Sidebar.svelte";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import AppDataProvider from "$lib/sections/AppDataProvider.svelte";
</script>

<!-- Global query client context -->
<QueryClientProvider client={queryClient}>
  <!-- App data loader and context provider -->
  <AppDataProvider>
    <main class="main">
      <Sidebar />
      <div class="content">
        {#if $navigating}
          <div class="skeleton-list">
            <div class="skeleton" style="width: 90%; height: 1.5rem;"></div>
            <div class="skeleton" style="width: 70%; height: 1rem;"></div>
            <div class="skeleton" style="width: 80%; height: 1rem;"></div>
          </div>
        {:else}
          <slot />
        {/if}
      </div>
    </main>
  </AppDataProvider>
</QueryClientProvider>

<!-- Toast popup provider -->
<AppToaster />

<style>
  .main {
    display: flex;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .content {
    display: flex;
    flex-flow: column;
    position: relative;
    flex: auto;
    height: 100%;
    overflow: hidden;
  }
</style>
