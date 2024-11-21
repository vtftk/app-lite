const PERSISTED_AUTH_TOKEN_KEY = "__VTUBE_STUDIO_API_KEY";

export function getPersistedAuthToken(): string | null {
  return localStorage.getItem(PERSISTED_AUTH_TOKEN_KEY);
}

export function setPersistedAuthToken(token: string | null) {
  if (token === null) {
    localStorage.removeItem(PERSISTED_AUTH_TOKEN_KEY);
    return;
  }

  localStorage.setItem(PERSISTED_AUTH_TOKEN_KEY, token);
}
