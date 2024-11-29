<script lang="ts">
  import type { Snippet } from "svelte";
  import PlayIcon from "~icons/solar/play-bold";
  import StopIcon from "~icons/solar/stop-bold";

  type Props = {
    src: string;

    button?: Snippet<[{ onClick: VoidFunction; isPlaying: boolean }]>;
  };

  let { src, button }: Props = $props();

  let audio: HTMLAudioElement | undefined = $state(undefined);
  let isPlaying = $state(false);

  function togglePlay() {
    if (!audio) return;

    if (isPlaying) {
      audio.pause();
      audio.currentTime = 0;
    } else {
      audio.play();
    }

    isPlaying = !isPlaying;
  }

  function updateProgress() {
    if (!audio) return;

    // Still playing as long as we aren't at the end of the duration
    isPlaying = isPlaying && audio.currentTime < audio.duration;
  }
</script>

<!-- Play/Pause Button -->
{#if button}
  {@render button({ onClick: togglePlay, isPlaying })}
{:else}
  <button
    class="btn"
    onclick={togglePlay}
    aria-pressed={isPlaying ? "true" : "false"}
    aria-label={isPlaying ? "Pause audio" : "Play audio"}
  >
    {#if isPlaying}
      <StopIcon />
    {:else}
      <PlayIcon />
    {/if}
  </button>
{/if}

<audio
  bind:this={audio}
  ontimeupdate={updateProgress}
  {src}
  aria-label="Audio file"
>
</audio>
