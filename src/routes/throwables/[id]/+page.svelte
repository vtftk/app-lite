<script lang="ts">
  import { page } from "$app/stores";
  import { createItemQuery } from "$lib/api/items";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";

  const id = $derived($page.params.id);
  const itemQuery = $derived(createItemQuery(id));
</script>

{#if $itemQuery.isLoading}
  Loading...
{:else if $itemQuery.data}
  <ThrowableForm existing={$itemQuery.data} />
{:else}
  {#snippet actions()}
    <a class="btn" href="/throwables">Back</a>
  {/snippet}

  <PageLayoutList
    title="Throwable Not Found"
    description="Unknown throwable"
    {actions}
  />
{/if}
