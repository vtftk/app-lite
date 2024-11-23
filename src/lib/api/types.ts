type Option<T> = T | null;

export type MinMax = {
  min: number;
  max: number;
};

export type AppData = {
  throwables_config: ThrowablesConfig;
  items_config: ItemsConfig;
  model_config: ModelConfig;
  vtube_studio: VTubeStudioConfig;
  models: Record<ModelId, ModelData>;

  items: ThrowableConfig[];
};

export type ThrowablesConfig = {
  duration: number;
  spin_speed: MinMax;
  throw_angle: MinMax;
  direction: ThrowDirection;
  impact_delay: number;
};

export enum ThrowDirection {
  Random = "Random",
  LeftOnly = "LeftOnly",
  RightOnly = "RightOnly",
}

export type ModelId = string;

export type ModelData = {
  x: MinMax;
  y: MinMax;
};

export type ItemsConfig = {
  global_volume: number;
  item_scale: MinMax;
};

export type ModelConfig = {
  model_return_time: number;
  eyes_on_hit: EyesMode;
};

export enum EyesMode {
  Unchanged = "Unchanged",
  Opened = "Opened",
  Closed = "Closed",
}

export type VTubeStudioConfig = {
  host: string;
  port: number;
};

export type ThrowableConfig = {
  name: string;
  image: ThrowableImageConfig;
  sound: Option<ImpactSoundConfig>;
};

export type ThrowableImageConfig = {
  src: string;
  weight: number;
  scale: number;
  pixelate: boolean;
};

export type ImpactSoundConfig = {
  src: string;
  volume: number;
};

export type RuntimeAppData = {
  model_id: string | null;
  vtube_studio_connected: boolean;
};
