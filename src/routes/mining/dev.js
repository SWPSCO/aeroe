import { invoke } from '@tauri-apps/api/core';

export async function getSupabaseSession() {
    const session = await invoke('supabase_session');
    return session;
}
