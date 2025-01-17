export * from "$shared/dataV2";
export * from "$shared/appData";
export * from "$shared/runtimeAppData";

// File types for file uploads
export enum StorageFolder {
  ThrowableImage = "ThrowableImage",
  ImpactSound = "ImpactSound",
  ImpactImage = "ImpactImage",
  Sound = "Sound",
}

export type VTubeStudioBroadcast = {
  apiName: string;
  apiVersion: string;
  timestamp: number;
  messageType: string;
  requestID: string;
  data: {
    active: boolean;
    port: number;
    instanceID: number;
    windowTitle: string;
  };
};
