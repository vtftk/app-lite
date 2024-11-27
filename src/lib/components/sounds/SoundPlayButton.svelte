<script lang="ts">
  import PlayIcon from "~icons/solar/play-bold";
  import StopIcon from "~icons/solar/stop-bold";

  type Props = {
    src: string;
  };

  let { src }: Props = $props();

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

<div>
  <!-- Play/Pause Button -->
  <button
    class="play-button"
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

  <audio
    bind:this={audio}
    ontimeupdate={updateProgress}
    {src}
    aria-label="Audio file"
  >
  </audio>
</div>

<style>
  .play-button {
    padding: 0.5rem;
    background-color: #333;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .play-button:hover {
    background-color: #444;
  }
</style>
