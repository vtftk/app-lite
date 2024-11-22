<script lang="ts">
  import { twitchAuthState } from "$lib/globalStores";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  async function getTwitchURL(): Promise<string> {
    return await invoke("get_twitch_oauth_uri");
  }

  async function openTwitchURL() {
    const success = await invoke<boolean>("open_twitch_oauth_uri");
    // TODO: Handle error
  }

  onMount(async () => {
    $twitchAuthState = await invoke<boolean>("is_authenticated");
    console.log($twitchAuthState);
  });
</script>

<main>
  <div class="container">
    <h1 class="title">Twitch Login</h1>
    <p class="message">
      You are not currently connected to <b>Twitch</b>, please visit the link
      below to allow access. Click "Open in browser" to open the link in your
      default browser.
    </p>
    {#await getTwitchURL() then twitchURL}
      <input class="url" type="text" readonly value={twitchURL} />
    {/await}

    <button onclick={openTwitchURL} class="button">Open in browser</button>
  </div>
</main>

<style>
  main {
    display: flex;
    flex-flow: column;
    width: 100%;
    height: 100vh;
    justify-content: center;
    align-items: center;
  }

  .container {
    display: flex;
    flex-flow: column;
    gap: 1rem;
    max-width: 34rem;
  }

  .title {
    color: #fff;
  }

  .url {
    font-size: 1.25rem;
    padding: 1rem;
    background-color: #333;
    border: 1px solid #fff;
    color: #fff;
    border-radius: 0.25rem;
  }

  .button {
    font-size: 1rem;
    padding: 1rem;
    background-color: #712880;
    border: none;
    color: #fff;
    border-radius: 0.25rem;
    align-self: flex-start;
  }

  .button:hover {
    background-color: #9731ac;
    cursor: pointer;
  }
</style>
