<script lang="ts">
  import { toast } from "svelte-sonner";
  import { derived } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";
  import { setClipboard } from "$lib/utils/browser";
  import { toastErrorMessage } from "$lib/utils/error";
  import { createIsAuthenticatedQuery } from "$lib/api/oauth";
  import { createModelDataQuery } from "$lib/api/calibration";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import {
    getRuntimeAppData,
    createOverlayURLQuery,
    createTwitchOAuthURLQuery,
    createDeriveModelCalibrated,
  } from "$lib/api/runtimeAppData";

  const runtimeAppData = getRuntimeAppData();
  const isAuthenticated = createIsAuthenticatedQuery();

  // Query for the overlay URL
  const overlayURLQuery = createOverlayURLQuery();

  // Query for the twitch OAuth URL
  const twitchOAuthURLQuery = createTwitchOAuthURLQuery();

  const modelDataQuery = createModelDataQuery();
  const modelData = derived(
    modelDataQuery,
    ($modelDataQuery) => $modelDataQuery.data ?? [],
  );

  // Model needs to be calibrated if not available here
  const isModelCalibrated = createDeriveModelCalibrated(
    modelData,
    runtimeAppData,
  );

  /**
   * Handler for clicking the "Copy Link" button to copy the overlay URL
   */
  function onCopyOverlay() {
    const overlayURL: string | undefined = $overlayURLQuery.data;

    if (overlayURL === undefined) return;

    const copyPromise = setClipboard(overlayURL);
    toast.promise(copyPromise, {
      loading: "Copying overlay URL...",
      success: "Copied overlay URL",
      error: toastErrorMessage("Failed to copy overlay URL"),
    });
  }

  /**
   * Handler for clicking the "Open In Browser" button to open the
   * Twitch OAuth link in browser
   */
  function onOpenTwitchURL() {
    const openPromise = invoke<boolean>("open_twitch_oauth_uri");

    toast.promise(openPromise, {
      loading: "Opening Twitch login...",
      success: "Opened in default browser",
      error: toastErrorMessage("Failed to open in default browser"),
    });
  }

  /**
   * Handle logging out from Twitch
   */
  function onLogoutTwitch() {
    const logoutPromise = invoke<void>("logout");

    toast.promise(logoutPromise, {
      loading: "Logging out...",
      success: "Logged out",
      error: toastErrorMessage("Failed to logout"),
    });
  }
</script>

<PageLayoutList title="Home" description="Details about your current setup">
  <div class="status">
    <div class="status-item">
      <div class="status-text">
        <h2>VTube Studio</h2>

        {#if $runtimeAppData.vtube_studio_connected}
          <p>Connected to VTube studio.</p>

          {#if $runtimeAppData.vtube_studio_auth}
            <p>Authenticated</p>

            {#if $isModelCalibrated}
              <p>
                Connected to VTube studio, model is calibrated. Ready to throw
                items
              </p>

              <div class="actions">
                <a class="btn" href="/calibration">Recalibrate Model</a>
              </div>
            {:else}
              <span class="warning">
                Current model is not calibrated, you must calibrate your model
                in order to throw items.
              </span>

              <div class="actions">
                <a class="btn" href="/calibration">Calibrate Model</a>
              </div>
            {/if}
          {:else}
            <p>
              Not Authenticated, please accept the access request prompt within
              VTube Studio
            </p>
          {/if}
        {:else}
          <p>
            Not connected to VTube studio, throwing items will not work. <br />
            Ensure you have the overlay setup in OBS and authorized in VTube Studio
          </p>
        {/if}
      </div>

      <div
        class="status-indicator"
        data-status={$runtimeAppData.vtube_studio_connected
          ? $isModelCalibrated && $runtimeAppData.vtube_studio_auth
            ? "green"
            : "orange"
          : "red"}
      ></div>
    </div>
    <div class="status-item">
      <div class="status-text">
        <h2>Active Overlay</h2>
        <p>Connected OBS overlays</p>

        {#if $overlayURLQuery.data}
          <div class="actions">
            <button class="btn" onclick={onCopyOverlay}>Copy Link</button>
            <p class="url">{$overlayURLQuery.data}</p>
          </div>
        {/if}
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
              <button class="btn" onclick={onLogoutTwitch}>Logout</button>
            </div>
          {:else}
            <p>
              You are not currently connected to <b>Twitch</b>, please visit the
              link below to allow access. Click "Open in browser" to open the
              link in your default browser.
            </p>

            <div class="actions">
              <button class="btn" onclick={onOpenTwitchURL}>
                Open in browser
              </button>

              {#if $twitchOAuthURLQuery.data}
                <input
                  class="url"
                  type="text"
                  readonly
                  value={$twitchOAuthURLQuery.data}
                />
              {/if}
            </div>
          {/if}
        </div>
        <div
          class="status-indicator"
          data-status={$isAuthenticated.data ? "green" : "red"}
        ></div>
      </div>
    {/if}
  </div>
</PageLayoutList>

<style>
  .status {
    display: flex;
    flex-flow: column;
    gap: 1rem;
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
    padding: 1rem;
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

  .warning {
    color: orange;
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
