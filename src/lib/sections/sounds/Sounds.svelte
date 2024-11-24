<script lang="ts">
  import type { SoundConfig } from "$lib/api/types";
  import SoundItem from "./SoundItem.svelte";

  type Props = {
    sounds: SoundConfig[];
  };

  const { sounds }: Props = $props();
</script>

<div class="grid">
  <a class="create" href="/sounds/create"> Create Sound </a>

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

  .create {
    background-color: #222;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;

    padding: 1rem;

    cursor: pointer;
    border: none;
    color: #fff;
    align-items: center;
    justify-content: center;
    font-size: 1em;
  }
</style>
