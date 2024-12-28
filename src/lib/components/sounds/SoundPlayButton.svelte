<script lang="ts">
  import type { Snippet } from "svelte";

  import PlayIcon from "~icons/solar/play-bold";
  import StopIcon from "~icons/solar/stop-bold";

  import Button from "../input/Button.svelte";

  type Props = {
    src: string;
    volume?: number;

    showText?: boolean;

    button?: Snippet<[{ onClick: VoidFunction; isPlaying: boolean }]>;
  };

  let { src, volume, showText, button }: Props = $props();

  let audio: HTMLAudioElement | undefined = $state(undefined);
  let isPlaying = $state(false);

  function togglePlay() {
    if (!audio) return;

    if (isPlaying) {
      audio.pause();
      audio.currentTime = 0;
    } else {
      audio.volume = volume ?? 1;
      audio.play();
    }

    isPlaying = !isPlaying;
  }

  function updateProgress() {
    if (!audio) return;

    // Still playing as long as we aren't at the end of the duration
    isPlaying = isPlaying && audio.currentTime < audio.duration;
  }

  $effect(() => {
    if (audio) audio.volume = volume ?? 1;
  });
</script>

<!-- Play/Pause Button -->
{#if button}
  {@render button({ onClick: togglePlay, isPlaying })}
{:else}
  <Button
    onclick={togglePlay}
    aria-pressed={isPlaying ? "true" : "false"}
    aria-label={isPlaying ? "Pause audio" : "Play audio"}
  >
    {#if isPlaying}
      <StopIcon />

      {#if showText}
        Stop
      {/if}
    {:else}
      <PlayIcon />

      {#if showText}
        Play
      {/if}
    {/if}
  </Button>
{/if}

<audio
  bind:this={audio}
  ontimeupdate={updateProgress}
  {src}
  aria-label="Audio file"
>
</audio>
