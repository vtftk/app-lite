<script lang="ts">
  import { page } from "$app/stores";
  import { createItemQuery } from "$lib/api/itemModel";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";

  const id = $derived($page.params.id);
  const itemQuery = $derived(createItemQuery(id));
</script>

{#if $itemQuery.isLoading}
  <div class="skeleton-list">
    <div class="skeleton" style="width: 90%; height: 1.5rem;"></div>
    <div class="skeleton" style="width: 70%; height: 1rem;"></div>
    <div class="skeleton" style="width: 80%; height: 1rem;"></div>
  </div>
{:else if $itemQuery.data}
  <ThrowableForm existing={$itemQuery.data} />
{:else}
  {#snippet actions()}
    <LinkButton href="/throwables">Back</LinkButton>
  {/snippet}

  <PageLayoutList
    title="Throwable Not Found"
    description="Unknown throwable"
    {actions}
  />
{/if}

<style>
  .skeleton-list {
    padding: 1rem;
  }
</style>
