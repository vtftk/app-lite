<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function getTwitchURL(): Promise<string> {
    return await invoke("get_twitch_oauth_uri");
  }

  async function openTwitchURL() {
    const success = await invoke<boolean>("open_twitch_oauth_uri");
    // TODO: Handle error
  }
</script>

}

<div>
  <h1>Twitch Login</h1>
  <p>You are not currently logged in with Twitch</p>

  {#await getTwitchURL() then twitchURL}
    <p>{twitchURL}</p>
  {/await}

  <button onclick={openTwitchURL}>Login</button>
</div>
