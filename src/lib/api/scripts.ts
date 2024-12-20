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

import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import { queryClient } from "./utils";

const SCRIPTS_KEY = ["scripts"];

function invalidateScriptsList() {
  return queryClient.invalidateQueries({ queryKey: SCRIPTS_KEY });
}

export function getScripts() {
  return invoke<Script[]>("get_scripts");
}

function createScriptKey(id: ScriptId) {
  return ["script", id] as const;
}

export function getScriptById(scriptId: ScriptId) {
  return invoke<Script | null>("get_script_by_id", { scriptId });
}

export async function createScript(create: CreateScript) {
  const script = await invoke<Script>("create_script", { create });

  // Invalidate the specific script query
  const scriptKey = createScriptKey(script.id);
  queryClient.setQueryData(scriptKey, script);

  invalidateScriptsList();

  return script;
}

export async function updateScript(
  update: UpdateScript,
  invalidateList = true,
) {
  const script = await invoke<Script>("update_script", update);

  // Invalidate the specific item query
  const itemKey = createScriptKey(script.id);
  queryClient.setQueryData(itemKey, script);

  if (invalidateList) invalidateScriptsList();

  return script;
}

export async function deleteScript(scriptId: ScriptId, invalidateList = true) {
  await invoke<void>("delete_script", { scriptId });

  const scriptKey = createScriptKey(scriptId);

  // Cancel any queries for the item and clear the current item data
  queryClient.cancelQueries({ queryKey: scriptKey });
  queryClient.setQueryData(scriptKey, undefined);

  if (invalidateList) invalidateScriptsList();
}

export async function bulkDeleteScripts(scriptIds: ScriptId[]) {
  await Promise.all(scriptIds.map((scriptId) => deleteScript(scriptId, false)));
  invalidateScriptsList();
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

export async function deleteScriptLogs(scriptId: ScriptId, logIds: LogId[]) {
  await invoke<void>("delete_script_logs", { logIds });

  queryClient.setQueriesData<ScriptLog[]>(
    { queryKey: createScriptLogsKey(scriptId) },
    (data) => {
      if (data === undefined) return undefined;
      return data.filter((log) => !logIds.includes(log.id));
    },
  );

  // Invalid the list of items
  queryClient.invalidateQueries({
    queryKey: createScriptLogsKey(scriptId),
  });
}

export async function updateScriptOrder(update: UpdateOrdering[]) {
  await invoke("update_script_orderings", { update });

  invalidateScriptsList();
}

// -----------------------------------------------------

export function createScriptsQuery() {
  return createQuery({
    queryKey: SCRIPTS_KEY,
    queryFn: getScripts,
  });
}

export function createScriptQuery(id: ScriptId) {
  return createQuery({
    queryKey: createScriptKey(id),
    queryFn: () => getScriptById(id),
  });
}

export function deleteScriptMutation() {
  return createMutation<void, Error, ScriptId>({
    mutationFn: deleteScript,
  });
}
