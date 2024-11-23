<script lang="ts">
  import "$lib/styles/app.css";

  import { queryClient } from "$lib/api/utils";
  import { QueryClientProvider } from "@tanstack/svelte-query";
</script>

<QueryClientProvider client={queryClient}>
  <slot />
</QueryClientProvider>
