<script>
  import { createIsAuthenticatedQuery } from "$lib/api/twitch";

  import TwitchOAuth from "./TwitchOAuth.svelte";

  const isAuthenticated = createIsAuthenticatedQuery();
</script>

{#if $isAuthenticated.isLoading}
  <p>Loading Twitch authentication...</p>
{:else if $isAuthenticated.data}
  <slot />
{:else}
  <TwitchOAuth />
{/if}
