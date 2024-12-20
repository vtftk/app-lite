<script lang="ts">
  import { page } from "$app/stores";
  import { createCommandQuery } from "$lib/api/commands";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import CommandForm from "$lib/sections/commands/CommandForm.svelte";

  const id = $derived($page.params.id);
  const commandQuery = $derived(createCommandQuery(id));
</script>

{#if $commandQuery.isLoading}
  Loading...
{:else if $commandQuery.data}
  <CommandForm existing={$commandQuery.data} />
{:else}
  {#snippet actions()}
    <a type="button" href="/commands">Back</a>
  {/snippet}

  <PageLayoutList
    title="Command Not Found"
    description="Unknown command"
    {actions}
  ></PageLayoutList>
{/if}
