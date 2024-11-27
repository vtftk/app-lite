<script lang="ts">
  import { tweened } from "svelte/motion";
  import PlayIcon from "~icons/solar/play-bold";
  import StopIcon from "~icons/solar/stop-bold";

  type Props = {
    src: string;
  };

  let { src }: Props = $props();

  let audio: HTMLAudioElement | undefined = $state(undefined);
  let isPlaying = $state(false);

  let progress = $state(0);

  // Play / Pause toggle
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

  // Update progress as the audio plays
  function updateProgress() {
    if (!audio) return;

    // Still playing as long as we aren't at the end of the duration
    isPlaying = isPlaying && audio.currentTime < audio.duration;

    progress = (audio.currentTime / audio.duration) * 100;
  }

  // Update the audio time when the user drags the progress bar
  function seek(event: MouseEvent) {
    if (!audio) return;
    const target = event.target;
    if (!target || !(target instanceof HTMLElement)) return;

    const newTime =
      (event.offsetX / (event.target as HTMLElement).offsetWidth) *
      audio.duration;
    audio.currentTime = newTime;
  }
</script>

<div class="sound-preview">
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

  <button
    class="progress-bar"
    role="slider"
    aria-valuemin="0"
    aria-valuemax="100"
    aria-valuenow={progress}
    aria-label="Audio progress"
    onmousedown={seek}
  >
    <div class="progress" style="width: {progress}%"></div>
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
  .sound-preview {
    display: flex;
    gap: 1rem;
    width: 300px;
  }

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

  .progress-bar {
    position: relative;
    width: 100%;
    height: 2rem;
    border: none;
    background-color: #333;
    border-radius: 0.25rem;
  }

  .progress {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    background-color: #555;
    transition: width 0.1s ease;
    border-radius: 0.25rem;
  }
</style>
