<script lang="ts">
  import type { Sound } from "$lib/api/types";

  import { toast } from "svelte-sonner";
  import getBackendURL from "$lib/utils/url";
  import { deleteSound } from "$lib/api/sounds";
  import { toastErrorMessage } from "$lib/utils/error";
  import { getAppData } from "$lib/api/runtimeAppData";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import Button from "$lib/components/input/Button.svelte";
  import SolarMenuDotsBold from "~icons/solar/menu-dots-bold";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import { confirmDialog } from "$lib/components/GlobalConfirmDialog.svelte";
  import SoundPlayButton from "$lib/components/sounds/SoundPlayButton.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";

  type Props = {
    config: Sound;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const appData = getAppData();

  const { config, selected, onToggleSelected }: Props = $props();

  async function onDelete() {
    const confirm = await confirmDialog({
      title: "Confirm Delete",
      description: "Are you sure you want to delete this sound?",
    });

    if (!confirm) {
      return;
    }

    const deletePromise = deleteSound(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting sound...",
      success: "Deleted sound",
      error: toastErrorMessage("Failed to delete sound"),
    });
  }
</script>

{#snippet popoverContent()}
  <SoundPlayButton
    showText
    src={getBackendURL(config.src)}
    volume={config.volume * $appData.sounds_config.global_volume}
  />
  <LinkButton href="/sounds/{config.id}">
    <SettingsIcon /> View
  </LinkButton>
  <Button onclick={onDelete}><DeleteIcon /> Delete</Button>
{/snippet}

<div class="sound">
  <ControlledCheckbox checked={selected} onCheckedChange={onToggleSelected} />

  <div class="sound__text">
    <a class="sound__name" href="/sounds/{config.id}">{config.name}</a>
  </div>

  <div class="action">
    <PopoverButton
      content={popoverContent}
      contentProps={{ align: "start", side: "left" }}
    >
      <SolarMenuDotsBold />
    </PopoverButton>
  </div>
</div>

<style>
  .sound {
    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    border-radius: 5px;

    display: flex;
    justify-content: space-between;
    gap: 1rem;

    padding: 0.5rem;
    align-items: center;
    overflow: hidden;
    height: 60px;
  }

  .sound__name {
    flex: 1;
    color: #fff;
    font-weight: bold;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    text-decoration: none;
  }

  .sound__name:hover {
    text-decoration: underline;
  }

  .sound__text {
    display: flex;
    flex: auto;
    align-items: center;
    overflow: hidden;
  }

  .action {
    flex-shrink: 0;
  }
</style>
