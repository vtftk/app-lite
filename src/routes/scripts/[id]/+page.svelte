<script lang="ts">
  import { page } from "$app/stores";
  import { getAppData } from "$lib/api/runtimeAppData";
  import type { UserScriptConfig } from "$lib/api/types";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ScriptForm from "$lib/sections/scripts/ScriptForm.svelte";
  import { derived, type Readable } from "svelte/store";

  const appData = getAppData();

  const item: Readable<UserScriptConfig | undefined> = derived(
    [appData, page],
    ([$appData, $page]) => {
      const id = $page.params.id;
      const item = $appData.scripts.find((item) => item.id === id);
      return item;
    }
  );
</script>

{#if $item !== undefined}
  {#snippet actions()}
    <a type="button" href="/scripts">Back</a>
  {/snippet}

  <PageLayoutList title="Edit Script" description="Editing a script" {actions}>
    <ScriptForm existing={$item} />
  </PageLayoutList>
{:else}
  {#snippet actions()}
    <a type="button" href="/scripts">Back</a>
  {/snippet}

  <PageLayoutList
    title="Script Not Found"
    description="Unknown script"
    {actions}
  ></PageLayoutList>
{/if}
