<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { ItemConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
  import { derived, type Readable } from "svelte/store";

  const appData = getAppData();

  const item: Readable<ItemConfig | undefined> = derived(
    [appData, page],
    ([$appData, $page]) => {
      const id = $page.params.id;
      const item = $appData.items.find((item) => item.id === id);
      return item;
    }
  );
</script>

{#if $item !== undefined}
  {#snippet actions()}
    <a class="btn" href="/throwables">Back</a>
  {/snippet}

  <PageLayoutList
    title="Edit Throwable"
    description="Editing a throwable"
    {actions}
  >
    <ThrowableForm existing={$item} />
  </PageLayoutList>
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
