export type RuntimeAppData = {
  model_id: string | null;
  vtube_studio_connected: boolean;
  hotkeys: VTubeStudioHotkey[];
};

export type VTubeStudioHotkey = {
  hotkey_id: string;
  name: string;
};
