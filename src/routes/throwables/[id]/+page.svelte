<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import { testThrow, testThrowBarrage } from "$lib/api/throwables";
  import type { ItemConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
  import { toast } from "svelte-sonner";
  import { derived, type Readable } from "svelte/store";
  import BallsIcon from "~icons/solar/balls-bold-duotone";
  import BallIcon from "~icons/solar/basketball-bold-duotone";

  const appData = getAppData();

  const item: Readable<ItemConfig | undefined> = derived(
    [appData, page],
    ([$appData, $page]) => {
      const id = $page.params.id;
      const item = $appData.items.find((item) => item.id === id);
      return item;
    }
  );

  function onTestThrow(config: ItemConfig) {
    const throwPromise = testThrow($appData, [config.id], 1);

    toast.promise(throwPromise, {
      loading: "Sending throw...",
      success: "Threw item",
      error: "Failed to throw item",
    });
  }

  function onTestBarrage(config: ItemConfig) {
    const throwPromise = testThrowBarrage($appData, [config.id], 50, 2, 100);

    toast.promise(throwPromise, {
      loading: "Sending barrage...",
      success: "Threw barrage",
      error: "Failed to throw barrage",
    });
  }
</script>

{#if $item !== undefined}
  {#snippet actions()}
    <button type="button" class="btn" onclick={() => onTestThrow($item)}>
      <BallIcon /> Test
    </button>
    <button type="button" class="btn" onclick={() => onTestBarrage($item)}>
      <BallsIcon /> Test Barrage
    </button>
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
