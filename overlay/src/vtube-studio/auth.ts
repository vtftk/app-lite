import { pluginDeveloper, pluginIcon, pluginName } from "./constants";
import { InvalidMessageTypeError } from "./error";
import { createVTubeMessage } from "./message";
import { sendSocketMessage } from "./socket";
import { getPersistedAuthToken, setPersistedAuthToken } from "./token";

let authToken: string | null = getPersistedAuthToken();

/**
 * Attempts to authorize with the VTube Studio socket
 *
 * @param attempt The attempt number
 * @returns Promise for the attempt result
 */
export async function attemptAuthorization(attempt: number = 1) {
  // Attempt to acquire a token if one is not available
  if (authToken === null) {
    authToken = await requestAuthenticationToken();
  }

  // Request authentication using the token
  const authenticated = await requestAuthentication(authToken);
  if (!authenticated) {
    // Clear stored token before next attempt
    setPersistedAuthToken(null);

    // Attempt to retry failed authentication (Old auth token may be expired)
    if (attempt > 1) throw new Error("failed to complete authentication");
    return await attemptAuthorization(attempt + 1);
  }

  // Persist successful auth token
  setPersistedAuthToken(authToken);
}

/**
 * Requests an authentication token from VTube studio, this will prompt
 * the user to accept or decline the request
 *
 * @returns The token on success
 */
async function requestAuthenticationToken(): Promise<string> {
  const request = createVTubeMessage("AuthenticationTokenRequest", {
    pluginName,
    pluginDeveloper,
    pluginIcon,
  });

  const response = await sendSocketMessage(request);
  if (response.messageType !== "AuthenticationTokenResponse") {
    throw new InvalidMessageTypeError(
      "AuthenticationTokenResponse",
      response.messageType
    );
  }

  if (!response.data) {
    throw new Error("missing response data");
  }

  const authenticationToken = response.data.authenticationToken;

  if (!authenticationToken) {
    throw new Error("missing authentication token");
  }

  return authenticationToken;
}

async function requestAuthentication(
  authenticationToken: string
): Promise<boolean> {
  const request = createVTubeMessage("AuthenticationRequest", {
    pluginName,
    pluginDeveloper,
    authenticationToken,
  });

  const response = await sendSocketMessage(request);
  if (response.messageType !== "AuthenticationResponse") {
    throw new InvalidMessageTypeError(
      "AuthenticationResponse",
      response.messageType
    );
  }

  if (!response.data) {
    throw new Error("missing response data");
  }

  return response.data.authenticated;
}
