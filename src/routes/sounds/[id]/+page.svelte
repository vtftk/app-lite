<script lang="ts">
  import { page } from "$app/stores";
  import { createSoundQuery } from "$lib/api/sounds";
  import SoundForm from "$lib/sections/sounds/SoundForm.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";

  const id = $derived($page.params.id);
  const soundQuery = $derived(createSoundQuery(id));
</script>

{#if $soundQuery.isLoading}
  Loading...
{:else if $soundQuery.data}
  <SoundForm existing={$soundQuery.data} />
{:else}
  {#snippet actions()}
    <a type="button" href="/sounds">Back</a>
  {/snippet}

  <PageLayoutList
    title="Sound Not Found"
    description="Unknown sound"
    {actions}
  />
{/if}
