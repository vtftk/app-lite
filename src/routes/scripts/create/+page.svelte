<script lang="ts">
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ScriptForm from "$lib/sections/scripts/ScriptForm.svelte";
</script>

{#snippet actions()}
  <a class="btn" href="/scripts">Back</a>
{/snippet}

<PageLayoutList
  title="Create Script"
  description="Create a script that can be triggered"
  {actions}
>
  <ScriptForm />
</PageLayoutList>
