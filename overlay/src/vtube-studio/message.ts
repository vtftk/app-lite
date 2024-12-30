export type VTubeMessage<D> = {
  apiName: string;
  apiVersion: string;
  requestID: string;
  messageType: string;
  data: D;
};

export function createVTubeMessage<D = undefined>(
  messageType: string,
  data: D,
): VTubeMessage<D> {
  return {
    apiName: "VTubeStudioPublicAPI",
    apiVersion: "1.0",
    requestID: "",
    messageType,
    data,
  };
}
