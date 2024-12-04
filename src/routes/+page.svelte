<script lang="ts">
  import { getAppData, getRuntimeAppData } from "$lib/api/runtimeAppData";
  import { derived } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";

  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import { createIsAuthenticatedQuery } from "$lib/api/oauth";

  const appData = getAppData();
  const runtimeAppData = getRuntimeAppData();
  const isAuthenticated = createIsAuthenticatedQuery();

  // Model needs to be calibrated if not available here
  const isModelCalibrated = derived(
    [appData, runtimeAppData],
    ([$appData, $runtimeAppData]) => {
      // No model active
      if ($runtimeAppData.model_id === null) {
        return false;
      }

      const modelData = $appData.models[$runtimeAppData.model_id];
      return modelData !== undefined;
    }
  );

  async function setClipboard(text: string) {
    const type = "text/plain";
    const blob = new Blob([text], { type });
    const data = [new ClipboardItem({ [type]: blob })];
    await navigator.clipboard.write(data);
  }

  async function getTwitchURL(): Promise<string> {
    return await invoke("get_twitch_oauth_uri");
  }

  async function getOverlayURL(): Promise<string> {
    return await invoke("get_overlay_url");
  }

  async function openTwitchURL() {
    await invoke<boolean>("open_twitch_oauth_uri");
  }

  async function logout() {
    await invoke("logout");
  }
</script>

<PageLayoutList title="Home" description="Details about your current setup">
  <div class="status">
    <div class="status-item">
      <div class="status-text">
        <h2>VTube Studio</h2>

        {#if $runtimeAppData.vtube_studio_connected}
          <p>Connected to VTube studio, ready to throw items</p>
        {:else}
          <p>
            Not connected to VTube studio, throwing items will not work. <br /> Ensure
            you have the overlay setup in OBS and authorized in VTube Studio
          </p>
        {/if}
      </div>

      <div
        class="status-indicator"
        data-status={$runtimeAppData.vtube_studio_connected ? "green" : "red"}
      ></div>
    </div>
    <div class="status-item">
      <div class="status-text">
        <h2>Active Overlay</h2>
        <p>Connected OBS overlays</p>

        {#await getOverlayURL() then overlayURL}
          <div class="actions">
            <button class="btn" onclick={() => setClipboard(overlayURL)}
              >Copy Link</button
            >
            <p class="url">{overlayURL}</p>
          </div>
        {/await}
      </div>
      <div
        class="status-indicator"
        data-status={$runtimeAppData.active_overlay_count > 0 ? "green" : "red"}
      >
        {$runtimeAppData.active_overlay_count}
      </div>
    </div>
    {#if $isAuthenticated.isLoading}
      <div class="status-item">
        <div class="status-text">
          <h2>Twitch</h2>
          <p>Checking twitch login status....</p>
        </div>
        <div class="status-indicator" data-status="yellow"></div>
      </div>
    {:else}
      <div
        class="status-item"
        data-status={$isAuthenticated.data ? "green" : "red"}
      >
        <div class="status-text">
          <h2>Twitch</h2>

          {#if $isAuthenticated.data}
            <p>Connected to your Twitch account.</p>

            <div class="actions">
              <button class="btn" onclick={logout}>Logout</button>
            </div>
          {:else}
            <p>
              You are not currently connected to <b>Twitch</b>, please visit the
              link below to allow access. Click "Open in browser" to open the
              link in your default browser.
            </p>

            <div class="actions">
              <button class="btn" onclick={openTwitchURL}
                >Open in browser</button
              >
              {#await getTwitchURL() then twitchURL}
                <input class="url" type="text" readonly value={twitchURL} />
              {/await}
            </div>
          {/if}
        </div>
        <div
          class="status-indicator"
          data-status={$isAuthenticated.data ? "green" : "red"}
        ></div>
      </div>
    {/if}
    <div class="status-item">
      <div class="status-text">
        <h2>Model Calibration</h2>
        {#if $runtimeAppData.vtube_studio_connected}
          <p>Calibrate model</p>
          <div class="actions">
            <a class="btn" href="/calibration">Calibrate Model</a>
          </div>
        {:else}
          <p>Not connected to VTube studio</p>{/if}
      </div>
      <div
        class="status-indicator"
        data-status={$isModelCalibrated
          ? "green"
          : $runtimeAppData.vtube_studio_connected
            ? "red"
            : "orange"}
      ></div>
    </div>
  </div>
</PageLayoutList>

<style>
  .status {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    margin: 0.5rem 0;
  }

  .status-text {
    flex: auto;
    margin-right: 1rem;
  }

  .status-item {
    display: flex;
    border: 1px solid #333;
    justify-content: space-between;
    padding: 0.5rem;
    align-items: center;
    border-radius: 0.5rem;
  }

  .status-item h2 {
    font-size: 1rem;
    color: #fff;
  }

  .status-indicator {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    color: #fff;
    width: 2rem;
    height: 2rem;
    background-color: black;
    border-radius: 1rem;
    flex-shrink: 0;
  }

  .status-indicator[data-status="green"] {
    background-color: green;
  }

  .status-indicator[data-status="orange"] {
    background-color: orange;
  }

  .status-indicator[data-status="red"] {
    background-color: red;
  }

  .status-item[data-status="red"] {
    background-color: #2f2222;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .url {
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #333;
    border-radius: 0.25rem;
    color: #fff;
    flex: auto;
  }
</style>
