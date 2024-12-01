<script lang="ts">
  import "$lib/styles/global.scss";
  import "$lib/api/events";

  import { queryClient } from "$lib/api/utils";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import AppDataProvider from "$lib/sections/AppDataProvider.svelte";
  import Sidebar from "$lib/components/nav/Sidebar.svelte";
</script>

<QueryClientProvider client={queryClient}>
  <AppDataProvider>
    <main class="main">
      <Sidebar />
      <div class="content"><slot /></div>
    </main>
  </AppDataProvider>
</QueryClientProvider>

<style>
  .main {
    display: flex;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .content {
    flex: auto;
    height: 100%;
  }
</style>
