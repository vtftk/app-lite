<script lang="ts">
  import "$lib/styles/app.css";

  import { queryClient } from "$lib/api/utils";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import AppDataProvider from "$lib/sections/AppDataProvider.svelte";
  import TwitchOAuthGate from "$lib/sections/twitch/TwitchOAuthGate.svelte";
</script>

<QueryClientProvider client={queryClient}>
  <AppDataProvider>
    <TwitchOAuthGate>
      <slot />
    </TwitchOAuthGate>
  </AppDataProvider>
</QueryClientProvider>
