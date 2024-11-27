<script lang="ts">
  import type { SoundConfig } from "$lib/api/types";
  import BulkSoundImport from "$lib/components/sounds/BulkSoundImport.svelte";
  import SoundItem from "./SoundItem.svelte";

  type Props = {
    sounds: SoundConfig[];
  };

  const { sounds }: Props = $props();
</script>

<div class="grid">
  <div class="actions">
    <a class="btn" href="/sounds/create"> Create Sound </a>

    <BulkSoundImport />
  </div>

  {#each sounds as event}
    <SoundItem config={event} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  .actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
  }
</style>
