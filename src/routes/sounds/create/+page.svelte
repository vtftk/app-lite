<script lang="ts">
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import SoundForm from "$lib/sections/sounds/SoundForm.svelte";
</script>

{#snippet actions()}
  <a class="btn" href="/sounds">Back</a>
{/snippet}

<PageLayoutList
  title="Create Sound"
  description="Create a sound that can be triggered"
  {actions}
>
  <SoundForm />
</PageLayoutList>
