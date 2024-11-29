<script lang="ts">
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ThrowableForm from "$lib/sections/throwables/ThrowableForm.svelte";
</script>

{#snippet actions()}
  <a class="btn" href="/throwables">Back</a>
{/snippet}

<PageLayoutList
  title="Create Throwable"
  description="Create a new item that can be thrown"
  {actions}
>
  <ThrowableForm />
</PageLayoutList>
