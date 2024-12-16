<script lang="ts">
  import type { Script } from "$lib/api/types";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";
  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import { deleteScriptMutation } from "$lib/api/scripts";
  import Button from "$lib/components/input/Button.svelte";
  import SolarMenuDotsBold from "~icons/solar/menu-dots-bold";
  import LinkButton from "$lib/components/input/LinkButton.svelte";
  import PopoverButton from "$lib/components/popover/PopoverButton.svelte";
  import ControlledCheckbox from "$lib/components/input/ControlledCheckbox.svelte";

  type Props = {
    config: Script;

    selected: boolean;
    onToggleSelected: VoidFunction;
  };

  const { config, selected, onToggleSelected }: Props = $props();

  const deleteScript = deleteScriptMutation();

  async function onDelete() {
    if (!confirm("Are you sure you want to delete this script?")) {
      return;
    }

    const deletePromise = $deleteScript.mutateAsync(config.id);

    toast.promise(deletePromise, {
      loading: "Deleting script...",
      success: "Deleted script",
      error: toastErrorMessage("Failed to delete script"),
    });
  }
</script>

{#snippet popoverContent()}
  <LinkButton href="/scripts/{config.id}">
    <SettingsIcon /> View
  </LinkButton>
  <Button onclick={onDelete}><DeleteIcon /> Delete</Button>
{/snippet}

<div class="item">
  <ControlledCheckbox checked={selected} onCheckedChange={onToggleSelected} />

  <div class="item__text">
    <a class="item__name" href="/scripts/{config.id}">{config.name}</a>
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
  .item {
    background-color: #1a1a1a;
    border: 1px solid #2f2f2f;
    border-radius: 5px;

    display: flex;
    justify-content: space-between;
    gap: 1rem;

    padding: 0.5rem;
    align-items: center;
    overflow: hidden;
  }

  .item__name {
    flex: 1;
    color: #fff;
    font-weight: bold;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .item__text {
    display: flex;
    flex: auto;
    align-items: center;
    overflow: hidden;
  }

  .action {
    flex-shrink: 0;
  }
</style>
