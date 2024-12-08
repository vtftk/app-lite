<script lang="ts">
  import { page } from "$app/stores";
  import { createItemQueryDerived } from "$lib/api/items";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
  import { derived } from "svelte/store";

  const id = derived(page, ($page) => $page.params.id);
  const itemQuery = createItemQueryDerived(id);
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
