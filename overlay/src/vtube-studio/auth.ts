import { pluginDeveloper, pluginIcon, pluginName } from "./constants";
import { InvalidMessageTypeError } from "./error";
import { createVTubeMessage } from "./message";
import { VTubeStudioWebSocket } from "./socket";
import { getPersistedAuthToken, setPersistedAuthToken } from "./token";

let authToken: string | null = getPersistedAuthToken();

/**
 * Attempts to authorize with the VTube Studio socket
 *
 * @param attempt The attempt number
 * @returns Promise for the attempt result
 */
export async function attemptAuthorization(
  socket: VTubeStudioWebSocket,
  attempt: number = 1
) {
  // Attempt to acquire a token if one is not available
  if (authToken === null) {
    authToken = await requestAuthenticationToken(socket);
  }

  // Request authentication using the token
  const authenticated = await requestAuthentication(socket, authToken);
  if (!authenticated) {
    // Clear stored token before next attempt
    setPersistedAuthToken(null);

    // Attempt to retry failed authentication (Old auth token may be expired)
    if (attempt > 1) throw new Error("failed to complete authentication");
    return await attemptAuthorization(socket, attempt + 1);
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
async function requestAuthenticationToken(
  socket: VTubeStudioWebSocket
): Promise<string> {
  const request = createVTubeMessage("AuthenticationTokenRequest", {
    pluginName,
    pluginDeveloper,
    pluginIcon,
  });

  const response = await socket.send(request);
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
  socket: VTubeStudioWebSocket,

  authenticationToken: string
): Promise<boolean> {
  const request = createVTubeMessage("AuthenticationRequest", {
    pluginName,
    pluginDeveloper,
    authenticationToken,
  });

  const response = await socket.send(request);
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
