<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TwitchOAuth from "../lib/sections/TwitchOAuth.svelte";
  import { twitchAuthState } from "$lib/globalStores";
  import "$lib/api/events";
  import Calibration from "$lib/sections/Calibration.svelte";
  import { createRuntimeAppDataQuery } from "$lib/api/runtimeAppData";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function testThrow() {
    await invoke("test_throw", {
      config: {
        name: "Heart",
        image: {
          pixelate: false,
          scale: 0.5,
          src: "https://clipartcraft.com/images/transparent-hearts-tiny-3.png",
          weight: 1,
        },
        sound: null,
      },
      amount: 11,
    });
  }

  const runtimeAppData = createRuntimeAppDataQuery();
</script>

<main class="container">
  {#if $runtimeAppData.isLoading}
    Loading data...
  {:else if $twitchAuthState}
    <Calibration />

    <button onclick={testThrow}>Test throw</button>
  {:else}
    <TwitchOAuth />
  {/if}
</main>
