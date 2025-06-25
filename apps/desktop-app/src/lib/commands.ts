import { invoke } from '@tauri-apps/api/core';


export const externalBinaryDir = async () => await invoke<string>('external_binary_dir');