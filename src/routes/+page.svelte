<script lang="ts">
  import { toast } from "svelte-sonner";
  import { invoke } from "@tauri-apps/api/core";
  import Card from "$lib/components/Card.svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import Label from "$lib/components/Label.svelte";
  import { setClipboard } from "$lib/utils/browser";
  import { toastErrorMessage } from "$lib/utils/error";
  import { debounce } from "$lib/utils/debounce.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import { createModelDataQuery } from "$lib/api/calibration";
  import { createIsAuthenticatedQuery } from "$lib/api/twitch";
  import CardSkeleton from "$lib/components/CardSkeleton.svelte";
  import PageLayoutList from "$lib/layouts/PageLayoutList.svelte";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import {
    getAppContext,
    getTwitchOAuthURI,
    isModelCalibrated,
    createOverlayURLQuery,
  } from "$lib/api/runtimeAppData";

  const appContext = getAppContext();
  const runtimeAppData = $derived(appContext.runtimeAppData);

  const isAuthenticated = createIsAuthenticatedQuery();

  // Query for the overlay URL
  const overlayURLQuery = createOverlayURLQuery();

  const modelDataQuery = createModelDataQuery();

  // Model needs to be calibrated if not available here
  const modelCalibrated = $derived(
    isModelCalibrated($modelDataQuery.data ?? [], runtimeAppData.model_id),
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

  // Consistent loading times to prevent flickering
  const isTwitchLoading = $derived.by(
    debounce(() => $isAuthenticated.isLoading, 300, true),
  );
  const isCalibrationLoading = $derived.by(
    debounce(() => $modelDataQuery.isLoading, 300, true),
  );
</script>

<PageLayoutList
  title="Home | VTFTK"
  description="Details about your current setup"
>
  {#snippet actions()}
    <LinkButton href="https://vtftk.pages.dev" target="_blank">Help</LinkButton
    >{/snippet}

  <div class="status">
    {#if isTwitchLoading}
      <CardSkeleton />
    {:else}
      <Card>
        <div class="status-item">
          <svg
            class="status-item-logo"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
          >
            <path
              fill="#9d9c9c"
              d="M11.571 4.714h1.715v5.143H11.57zm4.715 0H18v5.143h-1.714zM6 0L1.714 4.286v15.428h5.143V24l4.286-4.286h3.428L22.286 12V0zm14.571 11.143l-3.428 3.428h-3.429l-3 3v-3H6.857V1.714h13.714Z"
            />
          </svg>
          <div class="status-text">
            <h2>
              Twitch

              <span class="labels">
                {#if $isAuthenticated.data}
                  <Label color="green">Connected</Label>
                {:else}
                  <Label color="red">Not Connected</Label>
                {/if}
              </span>
            </h2>

            {#if $isAuthenticated.data}
              <div class="actions">
                <Button onclick={onLogoutTwitch}>Logout</Button>
              </div>
            {:else}
              <!-- Not authenticated -->
              <p>
                You are not currently connected to <b>Twitch</b>, please visit
                the link below to allow access. Click "Open in browser" to open
                the link in your default browser.
              </p>

              <div class="actions">
                {#await getTwitchOAuthURI()}
                  <div class="skeleton" style="width: 20%; height: 1rem"></div>
                {:then url}
                  <LinkButton href={url} target="_blank">
                    Open in browser
                  </LinkButton>
                  <input class="url" type="text" readonly value={url} />
                {/await}
              </div>
            {/if}
          </div>

          <div
            class="status-indicator"
            data-status={$isAuthenticated.isLoading
              ? "yellow"
              : $isAuthenticated.data
                ? "green"
                : "red"}
          ></div>
        </div>
      </Card>
    {/if}

    <Card>
      <div class="status-item">
        <svg
          class="status-item-logo"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            fill="#9d9c9c"
            d="M12 24C5.383 24 0 18.617 0 12S5.383 0 12 0s12 5.383 12 12s-5.383 12-12 12m0-22.891C5.995 1.109 1.11 5.995 1.11 12S5.995 22.89 12 22.89S22.89 18.005 22.89 12S18.005 1.109 12 1.109M6.182 5.99c.352-1.698 1.503-3.229 3.05-3.996c-.269.273-.595.483-.844.78c-1.02 1.1-1.48 2.692-1.199 4.156c.355 2.235 2.455 4.06 4.732 4.028c1.765.079 3.485-.937 4.348-2.468c1.848.063 3.645 1.017 4.7 2.548c.54.799.962 1.736.991 2.711c-.342-1.295-1.202-2.446-2.375-3.095a4.9 4.9 0 0 0-3.772-.425c-1.56.448-2.849 1.723-3.293 3.293c-.377 1.25-.216 2.628.377 3.772c-.825 1.429-2.315 2.449-3.932 2.756c-1.244.261-2.551.059-3.709-.464c1.036.302 2.161.355 3.191-.011a4.91 4.91 0 0 0 3.024-2.935c.556-1.49.345-3.261-.591-4.54c-.7-1.007-1.803-1.717-3.002-1.969c-.38-.068-.764-.098-1.148-.134c-.611-1.231-.834-2.66-.528-3.996z"
          />
        </svg>

        <div class="status-text">
          <h2>
            OBS Overlay

            <span class="labels">
              {#if runtimeAppData.active_overlay_count > 0}
                <Label color="green">Connected</Label>
              {:else}
                <Label color="red">Not Connected</Label>
              {/if}
            </span>
          </h2>

          {#if $overlayURLQuery.data}
            <div class="actions">
              <Button onclick={onCopyOverlay}>Copy Link</Button>
              <p class="url">{$overlayURLQuery.data}</p>
            </div>
          {/if}
        </div>
        <div
          class="status-indicator"
          data-status={runtimeAppData.active_overlay_count > 0
            ? "green"
            : "red"}
        >
          {runtimeAppData.active_overlay_count}
        </div>
      </div>
    </Card>

    {#if isCalibrationLoading}
      <CardSkeleton />
    {:else}
      <Card>
        <div class="status-item">
          <img
            class="status-item-logo"
            src="/vt-studio.png"
            alt="VTube Studio Logo"
          />
          <div class="status-text">
            <h2>
              VTube Studio

              <span class="labels">
                {#if runtimeAppData.vtube_studio_connected}
                  <Label color="green">Connected</Label>

                  {#if runtimeAppData.vtube_studio_auth}
                    <Label color="green">Authorized</Label>

                    {#if modelCalibrated}
                      <Label color="green">Calibrated</Label>
                    {:else}
                      <Label color="red">Not Calibrated</Label>
                    {/if}
                  {:else}
                    <Label color="red">Not Authorized</Label>
                  {/if}
                {:else}
                  <Label color="red">Not Connected</Label>
                {/if}
              </span>
            </h2>

            {#if runtimeAppData.vtube_studio_connected}
              {#if runtimeAppData.vtube_studio_auth}
                {#if modelCalibrated}
                  <div class="actions">
                    <LinkButton href="/calibration"
                      >Recalibrate Model</LinkButton
                    >
                  </div>
                {:else}
                  <span class="warning">
                    Current model is not calibrated, you must calibrate your
                    model in order to throw items.
                  </span>

                  <div class="actions">
                    <LinkButton href="/calibration">Calibrate Model</LinkButton>
                  </div>
                {/if}
              {:else}
                <p>
                  Not Authenticated, please accept the access request prompt
                  within VTube Studio
                </p>
              {/if}
            {:else}
              <p>
                Not connected to VTube studio, throwing items will not work. <br
                />
                Ensure you have the overlay setup in OBS and authorized in VTube
                Studio
              </p>
            {/if}
          </div>

          <div
            class="status-indicator"
            data-status={runtimeAppData.vtube_studio_connected
              ? modelCalibrated && runtimeAppData.vtube_studio_auth
                ? "green"
                : "orange"
              : "red"}
          ></div>
        </div>
      </Card>
    {/if}

    <div class="creator-block">
      <img
        style="display: inline-block;vertical-align: middle;"
        width="64"
        height="64"
        src="/avatar-64x64.png"
        alt=""
      />
      <div class="creator-text">
        <p>
          Created by
          <a
            class="creator"
            href="https://github.com/jacobtread"
            target="_blank"
          >
            Jacobtread
          </a>
        </p>
        <p class="version">
          Version:

          {#await getVersion() then version}
            {version}
          {/await}
        </p>
      </div>
    </div>
  </div>
</PageLayoutList>

<style>
  .labels {
    display: inline-flex;
    flex-flow: row;
    gap: 0.5rem;
    margin-left: 0.5rem;
  }

  .creator-block {
    display: flex;
    align-items: center;
  }

  .version {
    font-size: 0.8rem;
    color: #999;
  }

  .status-item-logo {
    width: 48px;
    margin-right: 1.5rem;
    flex-shrink: 0;
    flex-grow: 0;
  }

  .creator {
    color: #e66c6c;
    font-weight: bold;
  }

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
    justify-content: space-between;
    align-items: center;
  }

  .status-text h2 {
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
