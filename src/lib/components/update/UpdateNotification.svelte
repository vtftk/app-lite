<script lang="ts">
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { getAppContext } from "$lib/api/runtimeAppData";
  import { check, Update } from "@tauri-apps/plugin-updater";

  const appContext = getAppContext();
  const appData = $derived(appContext.appData);

  async function checkUpdate(automatic: boolean) {
    let update: Update | null = null;
    try {
      update = await check();
    } catch (err) {
      console.error("failed to check for update:", err);
      return;
    }

    if (!update) return;

    const newVersion = update.version;

    if (automatic) {
      installUpdate(update);
    } else {
      toast("An update is available v" + newVersion, {
        action: {
          label: "Update",
          onClick: () => installUpdate(update),
        },
      });
    }
  }

  async function installUpdate(update: Update) {
    const updatePromise = update.downloadAndInstall();

    toast.promise(updatePromise, {
      loading: `Downloading and installing update v${update.version}...`,
      success: "Update and install complete",
      error: toastErrorMessage("Failed to download and install update"),
    });

    await updatePromise;

    toast("Restart to install the update", {
      duration: Infinity,
      action: {
        label: "Restart",
        onClick: () => {
          relaunch();
        },
      },
    });
  }

  $effect(() => {
    checkUpdate(appData.main_config.auto_updating);
  });
</script>
