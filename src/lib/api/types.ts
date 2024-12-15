export * from "$shared/dataV2";
export * from "$shared/appData";
export * from "$shared/runtimeAppData";

// File types for file uploads
export enum FileType {
  ThrowableImage = "ThrowableImage",
  ImpactSound = "ImpactSound",
  ImpactImage = "ImpactImage",
  Sound = "Sound",
}
