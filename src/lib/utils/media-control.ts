import { invoke } from '@tauri-apps/api/core'

export const togglePlayMedia = async () => await invoke('play_pause_media')
export const prevMedia = async () => await invoke('prev_media')
export const nextMedia = async () => await invoke('next_media')
