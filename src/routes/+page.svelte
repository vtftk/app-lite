<script lang="ts">
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import ObsHomeCard from "$lib/sections/home/OBSHomeCard.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import CreatorSection from "$lib/sections/home/CreatorSection.svelte";
  import TwitchHomeCard from "$lib/sections/home/TwitchHomeCard.svelte";
  import VTubeStudioCard from "$lib/sections/home/VTubeStudioCard.svelte";
</script>

<PageLayoutList
  title="Home | VTFTK"
  description="Details about your current setup"
>
  {#snippet actions()}
    <LinkButton href="https://vtftk.pages.dev" target="_blank">Help</LinkButton>
  {/snippet}

  <div class="cards">
    <TwitchHomeCard />
    <ObsHomeCard />
    <VTubeStudioCard />
  </div>

  <CreatorSection />
</PageLayoutList>

<style>
  .cards {
    display: flex;
    flex-flow: column;
    gap: 1rem;

    margin-top: 0.5rem;
    margin-bottom: 1rem;
  }
</style>
