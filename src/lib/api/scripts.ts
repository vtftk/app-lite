import type {
  ScriptId,
  Script,
  UpdateScript,
  CreateScript,
} from "$shared/dataV2";
import { createMutation, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";
import { queryClient } from "./utils";
import { derived, type Readable } from "svelte/store";

const SCRIPTS_KEY = ["scripts"];

export function getScripts() {
  return invoke<Script[]>("get_scripts");
}

export function createScriptsQuery() {
  return createQuery({
    queryKey: SCRIPTS_KEY,
    queryFn: getScripts,
  });
}

function createScriptKey(id: ScriptId) {
  return ["script", id] as const;
}

export function createScriptQuery(id: ScriptId | Readable<ScriptId>) {
  if (typeof id === "string") {
    return createQuery({
      queryKey: createScriptKey(id),
      queryFn: () => getScriptById(id),
    });
  }

  // Create query derived from ID store
  return createQuery(
    derived(id, (id) => ({
      queryKey: createScriptKey(id),
      queryFn: () => getScriptById(id),
    }))
  );
}

export function getScriptById(soundId: ScriptId) {
  return invoke<Script | null>("get_script_by_id", { soundId });
}

function createScript(create: CreateScript) {
  return invoke<Script>("create_script", { create });
}

export function createScriptMutation() {
  return createMutation<Script, Error, CreateScript>({
    mutationFn: (createItem) => createScript(createItem),

    onSuccess: (data) => {
      // Invalidate the specific sound query
      const soundKey = createScriptKey(data.id);
      queryClient.setQueryData(soundKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

export function bulkCreateScriptMutation() {
  return createMutation<Script[], Error, CreateScript[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createScript)),
    onSuccess: (sounds) => {
      for (const sound of sounds) {
        // Invalidate the specific sound query
        const soundKey = createScriptKey(sound.id);
        queryClient.setQueryData(soundKey, sound);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of sounds
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

function updateScript(scriptId: ScriptId, update: UpdateScript["update"]) {
  return invoke<Script>("update_script", { scriptId, update });
}

export function updateScriptMutation() {
  return createMutation<Script, Error, UpdateScript>({
    mutationFn: (update) => updateScript(update.scriptId, update.update),
    onSuccess: (data) => {
      // Invalidate the specific item query
      const itemKey = createScriptKey(data.id);
      queryClient.setQueryData(itemKey, data);
    },
    onSettled: (_data, _err, _updateItem) => {
      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

function deleteScript(scriptId: ScriptId) {
  return invoke<void>("delete_script", { scriptId });
}

export function deleteScriptMutation() {
  return createMutation<void, Error, ScriptId>({
    mutationFn: (scriptId) => deleteScript(scriptId),
    onMutate: (scriptId) => {
      const soundKey = createScriptKey(scriptId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: soundKey });
      queryClient.setQueryData(soundKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const soundKey = createScriptKey(itemId);
      queryClient.invalidateQueries({ queryKey: soundKey });

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

type BulkDeleteScripts = {
  scriptIds: ScriptId[];
};

export function bulkDeleteScriptMutation() {
  return createMutation<void[], Error, BulkDeleteScripts>({
    mutationFn: (deleteSounds) =>
      Promise.all(deleteSounds.scriptIds.map(deleteScript)),
    onMutate: (deleteSounds) => {
      for (const scriptId of deleteSounds.scriptIds) {
        const soundKey = createScriptKey(scriptId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: soundKey });
        queryClient.setQueryData(soundKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const scriptId of deleteItems.scriptIds) {
        // Invalidate the specific item query
        const soundKey = createScriptKey(scriptId);
        queryClient.invalidateQueries({ queryKey: soundKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}
