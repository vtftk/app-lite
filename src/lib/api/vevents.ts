import type {
  EventId,
  VEventData,
  UpdateEvent,
  CreateEvent,
  ExecutionId,
  UpdateOrdering,
  EventExecution,
  VEvent as Event,
  ExecutionsQuery,
} from "$shared/dataV2";

import { invoke } from "@tauri-apps/api/core";
import { derived, type Readable } from "svelte/store";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import { queryClient } from "./utils";

const EVENTS_KEY = ["events"];

export function getEvents() {
  return invoke<Event[]>("get_events");
}

export function createEventsQuery() {
  return createQuery({
    queryKey: EVENTS_KEY,
    queryFn: getEvents,
  });
}

function createEventKey(id: EventId) {
  return ["event", id] as const;
}

export function createEventQuery(id: EventId | Readable<EventId>) {
  if (typeof id === "string") {
    return createQuery({
      queryKey: createEventKey(id),
      queryFn: () => getEventById(id),
    });
  }

  // Create query derived from ID store
  return createQuery(
    derived(id, (id) => ({
      queryKey: createEventKey(id),
      queryFn: () => getEventById(id),
    })),
  );
}

export function getEventById(eventId: EventId) {
  return invoke<Event | null>("get_event_by_id", { eventId });
}

function createEvent(create: CreateEvent) {
  return invoke<Event>("create_event", { create });
}

export function createEventMutation() {
  return createMutation<Event, Error, CreateEvent>({
    mutationFn: (createItem) => createEvent(createItem),

    onSuccess: (data) => {
      // Invalidate the specific event query
      const eventKey = createEventKey(data.id);
      queryClient.setQueryData(eventKey, data);
    },
    onSettled: (_data, _err, _createItem) => {
      // Invalid the list of events
      queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
    },
  });
}

export function bulkCreateEventMutation() {
  return createMutation<Event[], Error, CreateEvent[]>({
    mutationFn: (createItems) => Promise.all(createItems.map(createEvent)),
    onSuccess: (events) => {
      for (const event of events) {
        // Invalidate the specific event query
        const eventKey = createEventKey(event.id);
        queryClient.setQueryData(eventKey, event);
      }
    },
    onSettled: (_data, _err, _createSound) => {
      // Invalid the list of events
      queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
    },
  });
}

function updateEvent(eventId: EventId, update: UpdateEvent["update"]) {
  return invoke<Event>("update_event", { eventId, update });
}

export function testEvent(eventId: EventId, eventData: VEventData) {
  return invoke<Event>("test_event_by_id", { eventId, eventData });
}

export function updateEventMutation() {
  return createMutation<Event, Error, UpdateEvent>({
    mutationFn: (update) => updateEvent(update.eventId, update.update),
    onSuccess: (data) => {
      // Invalidate the specific item query
      const itemKey = createEventKey(data.id);
      queryClient.setQueryData(itemKey, data);
    },
    onSettled: (_data, _err, _updateItem) => {
      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
    },
  });
}

function deleteEvent(eventId: EventId) {
  return invoke<void>("delete_event", { eventId });
}

export function deleteEventMutation() {
  return createMutation<void, Error, EventId>({
    mutationFn: (eventId) => deleteEvent(eventId),
    onMutate: (eventId) => {
      const eventKey = createEventKey(eventId);

      // Cancel any queries for the item and clear the current item data
      queryClient.cancelQueries({ queryKey: eventKey });
      queryClient.setQueryData(eventKey, undefined);

      return undefined;
    },
    onSettled: (_data, _err, itemId) => {
      // Invalidate the specific item query
      const eventKey = createEventKey(itemId);
      queryClient.invalidateQueries({ queryKey: eventKey });

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
    },
  });
}

type BulkDeleteEvents = {
  eventIds: EventId[];
};

export function bulkDeleteEventMutation() {
  return createMutation<void[], Error, BulkDeleteEvents>({
    mutationFn: (deleteSounds) =>
      Promise.all(deleteSounds.eventIds.map(deleteEvent)),
    onMutate: (deleteSounds) => {
      for (const eventId of deleteSounds.eventIds) {
        const eventKey = createEventKey(eventId);

        // Cancel any queries for the item and clear the current item data
        queryClient.cancelQueries({ queryKey: eventKey });
        queryClient.setQueryData(eventKey, undefined);
      }

      return undefined;
    },
    onSettled: (_data, _err, deleteItems) => {
      for (const eventId of deleteItems.eventIds) {
        // Invalidate the specific item query
        const eventKey = createEventKey(eventId);
        queryClient.invalidateQueries({ queryKey: eventKey });

        // Invalid the list of items
        queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
      }

      // Invalid the list of items
      queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
    },
  });
}

export async function updateEventOrder(update: UpdateOrdering[]) {
  await invoke("update_event_orderings", { update });
  queryClient.invalidateQueries({ queryKey: EVENTS_KEY });
}

function createEventExecutionsKey(id: EventId, query?: ExecutionsQuery) {
  if (query === undefined) {
    return ["event-executions", id] as const;
  }
  return ["event-executions", id, query] as const;
}

export function getEventExecutions(eventId: EventId, query: ExecutionsQuery) {
  return invoke<EventExecution[]>("get_event_executions", {
    eventId,
    query,
  });
}

export function eventExecutionsQuery(eventId: EventId, query: ExecutionsQuery) {
  return createQuery({
    queryKey: createEventExecutionsKey(eventId, query),
    queryFn: () => getEventExecutions(eventId, query),
  });
}

export function invalidateEventExecutions(
  eventId: EventId,
  query: ExecutionsQuery,
) {
  const queryKey = createEventExecutionsKey(eventId, query);
  queryClient.invalidateQueries({ queryKey });
}

export function deleteEventExecutions(executionIds: ExecutionId[]) {
  return invoke<void>("delete_event_executions", { executionIds });
}

type BulkDeleteCommandExecutions = {
  executionIds: ExecutionId[];
};

export function deleteEventExecutionsMutation(eventId: EventId) {
  return createMutation<void, Error, BulkDeleteCommandExecutions>({
    mutationFn: (deleteLogs) => deleteEventExecutions(deleteLogs.executionIds),
    onMutate: (deleteLogs) => {
      queryClient.setQueriesData<EventExecution[]>(
        { queryKey: createEventExecutionsKey(eventId) },
        (data) => {
          if (data === undefined) return undefined;
          return data.filter(
            (log) => !deleteLogs.executionIds.includes(log.id),
          );
        },
      );

      return undefined;
    },
    onSettled: (_data, _err) => {
      // Invalid the list of items
      queryClient.invalidateQueries({
        queryKey: createEventExecutionsKey(eventId),
      });
    },
  });
}
