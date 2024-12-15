import { invoke } from "@tauri-apps/api/core";
import { derived, type Readable } from "svelte/store";
import { createQuery, createMutation } from "@tanstack/svelte-query";
import type {
  LogId,
  Script,
  ScriptId,
  LogsQuery,
  ScriptLog,
  UpdateScript,
  CreateScript,
  UpdateOrdering,
} from "$shared/dataV2";

import { queryClient } from "./utils";

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

export function getScriptById(scriptId: ScriptId) {
  return invoke<Script | null>("get_script_by_id", { scriptId });
}

function createScript(create: CreateScript) {
  return invoke<Script>("create_script", { create });
}

export function createScriptMutation() {
  return createMutation<Script, Error, CreateScript>({
    mutationFn: (createItem) => createScript(createItem),

    onSuccess: (data) => {
      // Invalidate the specific script query
      const scriptKey = createScriptKey(data.id);
      queryClient.setQueryData(scriptKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of scripts
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

export function bulkCreateScriptMutation() {
  return createMutation<Script[], Error, CreateScript[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createScript)),
    onSuccess: (scripts) => {
      for (const script of scripts) {
        // Invalidate the specific script query
        const scriptKey = createScriptKey(script.id);
        queryClient.setQueryData(scriptKey, script);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of scripts
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
      const scriptKey = createScriptKey(scriptId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: scriptKey });
      queryClient.setQueryData(scriptKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const scriptKey = createScriptKey(itemId);
      queryClient.invalidateQueries({ queryKey: scriptKey });

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
        const scriptKey = createScriptKey(scriptId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: scriptKey });
        queryClient.setQueryData(scriptKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const scriptId of deleteItems.scriptIds) {
        // Invalidate the specific item query
        const scriptKey = createScriptKey(scriptId);
        queryClient.invalidateQueries({ queryKey: scriptKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
    },
  });
}

function createScriptLogsKey(id: ScriptId, query?: LogsQuery) {
  if (query === undefined) {
    return ["script-logs", id] as const;
  }

  return ["script-logs", id, query] as const;
}

export function getScriptLogs(scriptId: ScriptId, query: LogsQuery) {
  return invoke<ScriptLog[]>("get_script_logs", { scriptId, query });
}

export function invalidateScriptLogs(scriptId: ScriptId, query: LogsQuery) {
  const queryKey = createScriptLogsKey(scriptId, query);
  queryClient.invalidateQueries({ queryKey });
}

export function scriptLogsQuery(scriptId: ScriptId, query: LogsQuery) {
  return createQuery({
    queryKey: createScriptLogsKey(scriptId, query),
    queryFn: () => getScriptLogs(scriptId, query),
  });
}

export function deleteScriptLogs(logIds: LogId[]) {
  return invoke<void>("delete_script_logs", { logIds });
}

type BulkDeleteScriptLogs = {
  logIds: LogId[];
};

export function bulkDeleteScriptLogsMutation(scriptId: ScriptId) {
  return createMutation<void, Error, BulkDeleteScriptLogs>({
    mutationFn: (deleteLogs) => deleteScriptLogs(deleteLogs.logIds),
    onMutate: (deleteLogs) => {
      queryClient.setQueriesData<ScriptLog[]>(
        { queryKey: createScriptLogsKey(scriptId) },
        (data) => {
          if (data === undefined) return undefined;
          return data.filter((log) => !deleteLogs.logIds.includes(log.id));
        }
      );

      return undefined;
    },
    onSettled: (_data, _err) => {
      // Invalid the list of items
      queryClient.invalidateQueries({
        queryKey: createScriptLogsKey(scriptId),
      });
    },
  });
}

export async function updateScriptOrder(update: UpdateOrdering[]) {
  await invoke("update_script_orderings", { update });
  queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
}
