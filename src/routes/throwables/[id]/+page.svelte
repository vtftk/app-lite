<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { ItemConfig } from "$lib/api/types";
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
  <div class="container">
    <h1 class="title">Edit Throwable</h1>
    <p class="text">Edit</p>
    <a type="button" href="/throwables">Back</a>

    <ThrowableForm existing={$item} />
  </div>
{:else}
  <div class="container">
    <h1 class="title">Throwable Not Found</h1>
    <p class="text">Unknown throwable</p>
    <a type="button" href="/throwables">Back</a>
  </div>
{/if}

<style>
  .container {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;

    padding: 1rem;
  }

  .title {
    line-height: 1;
  }
</style>
