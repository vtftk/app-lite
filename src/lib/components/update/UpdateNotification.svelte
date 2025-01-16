<script lang="ts">
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
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
    const updatePromise = update.download();

    toast.promise(updatePromise, {
      loading: `Downloading update v${update.version}...`,
      success: "Update downloaded",
      error: toastErrorMessage("Failed to download update"),
    });

    await updatePromise;

    toast("Install the update (Will restart VTFTK)", {
      duration: Infinity,
      action: {
        label: "Install",
        onClick: () => {
          update.install();
        },
      },
    });
  }

  $effect(() => {
    checkUpdate(appData.main_config.auto_updating);
  });
</script>
