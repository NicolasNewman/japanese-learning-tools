import { invoke } from '@tauri-apps/api/core';


export const externalBinaryDir = async () => await invoke<string>('external_binary_dir');
export const openDevTools = async () => await invoke<string>('open_devtools');