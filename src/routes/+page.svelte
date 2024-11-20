<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TwitchOAuth from "../lib/sections/TwitchOAuth.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  import { authState } from "$lib/api/socket";
</script>

<main class="container">
  {#if $authState}
    <p>Test</p>
  {:else}
    <TwitchOAuth />
  {/if}
</main>
