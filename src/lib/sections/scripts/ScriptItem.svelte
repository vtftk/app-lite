<script lang="ts">
  import type { Script } from "$lib/api/types";

  import SettingsIcon from "~icons/solar/settings-bold";
  import DeleteIcon from "~icons/solar/trash-bin-2-bold";
  import { Checkbox } from "bits-ui";
  import { deleteScriptMutation } from "$lib/api/scripts";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/utils/error";

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

<div class="item">
  <Checkbox.Root checked={selected} onCheckedChange={onToggleSelected}>
    <Checkbox.Indicator let:isChecked>
      {#if isChecked}
        <span>&#10003;</span>
      {/if}
    </Checkbox.Indicator>
  </Checkbox.Root>

  <a class="name" href="/scripts/{config.id}">{config.name}</a>

  <div class="actions">
    <a class="btn" href="/scripts/{config.id}">
      <SettingsIcon />
    </a>
    <button class="btn" onclick={onDelete}> <DeleteIcon /> </button>
  </div>
</div>

<style>
  .item {
    background-color: #222;
    border: 1px solid #333;

    display: flex;
    justify-content: space-between;
    gap: 1rem;

    padding: 0.5rem;
    align-items: center;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .name {
    flex: 1;
    color: #fff;
    font-weight: bold;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
</style>
