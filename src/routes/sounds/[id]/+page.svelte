<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { SoundConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import SoundForm from "$lib/sections/sounds/SoundForm.svelte";
  import { derived, type Readable } from "svelte/store";

  const appData = getAppData();

  const item: Readable<SoundConfig | undefined> = derived(
    [appData, page],
    ([$appData, $page]) => {
      const id = $page.params.id;
      const item = $appData.sounds.find((item) => item.id === id);
      return item;
    }
  );
</script>

{#if $item !== undefined}
  {#snippet actions()}
    <a type="button" href="/sounds">Back</a>
  {/snippet}

  <PageLayoutList title="Edit Sound" description="Editing a sound" {actions}>
    <SoundForm existing={$item} />
  </PageLayoutList>
{:else}
  {#snippet actions()}
    <a type="button" href="/sounds">Back</a>
  {/snippet}

  <PageLayoutList title="Sound Not Found" description="Unknown sound" {actions}>
    <SoundForm existing={$item} />
  </PageLayoutList>
{/if}
