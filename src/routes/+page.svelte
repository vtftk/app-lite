<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TwitchOAuth from "../lib/sections/TwitchOAuth.svelte";
  import { twitchAuthState } from "$lib/globalStores";
  import "$lib/api/events";
  import Calibration from "$lib/sections/Calibration.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  {#if $twitchAuthState}
    <Calibration />
  {:else}
    <TwitchOAuth />
  {/if}
</main>
