<script lang="ts">
  // Apply global styling
  import "$lib/styles/global.scss";

  // Listen to events over IPC
  import "$lib/api/events";

  // Font family for code editor
  import "@fontsource/jetbrains-mono";

  import { Toaster } from "svelte-sonner";
  import { queryClient } from "$lib/api/utils";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import AppDataProvider from "$lib/sections/AppDataProvider.svelte";
  import Sidebar from "$lib/components/nav/Sidebar.svelte";
</script>

<!-- Global query client context -->
<QueryClientProvider client={queryClient}>
  <!-- App data loader and context provider -->
  <AppDataProvider>
    <main class="main">
      <Sidebar />
      <div class="content"><slot /></div>
    </main>
  </AppDataProvider>
</QueryClientProvider>

<!-- Toast popup provider -->
<Toaster
  theme="dark"
  richColors
  closeButton
  position="bottom-center"
  visibleToasts={3}
  duration={1000}
/>

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
