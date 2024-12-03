<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { ItemConfig, ThrowableConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
  import { invoke } from "@tauri-apps/api/core";
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

  async function testThrow(config: ItemConfig) {
    const impact_sounds = $appData.sounds.filter((sound) =>
      config.impact_sounds_ids.includes(sound.id)
    );

    const throwable: ThrowableConfig = {
      items: [config],
      impact_sounds,
    };

    await invoke("test_throw", {
      config: throwable,
      amount: 1,
    });
  }

  async function testThrowMany(config: ItemConfig) {
    const impact_sounds = $appData.sounds.filter((sound) =>
      config.impact_sounds_ids.includes(sound.id)
    );

    const throwable: ThrowableConfig = {
      items: [config],
      impact_sounds,
    };

    await invoke("test_throw_barrage", {
      config: throwable,
      amount: 50,
      amountPerThrow: 2,
      frequency: 100,
    });
  }
</script>

{#if $item !== undefined}
  {#snippet actions()}
    <button type="button" class="btn" onclick={() => testThrow($item)}>
      <BallIcon /> Test
    </button>
    <button type="button" class="btn" onclick={() => testThrowMany($item)}>
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
