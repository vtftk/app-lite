<script lang="ts">
  import "$lib/styles/app.css";
  import "$lib/api/events";

  import { queryClient } from "$lib/api/utils";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import AppDataProvider from "$lib/sections/AppDataProvider.svelte";
  import TwitchOAuthGate from "$lib/sections/twitch/TwitchOAuthGate.svelte";
  import Sidebar from "$lib/components/nav/Sidebar.svelte";
</script>

<QueryClientProvider client={queryClient}>
  <AppDataProvider>
    <TwitchOAuthGate>
      <main class="main">
        <Sidebar />
        <div class="content"><slot /></div>
      </main>
    </TwitchOAuthGate>
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
    background-color: #000;
  }
</style>
