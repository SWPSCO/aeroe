import { writable, type Writable } from 'svelte/store';

export const previousPage: Writable<string> = writable('/');

export const walletLoaded: Writable<boolean> = writable(false);
export const walletBalance: Writable<number | null> = writable(null);
export const walletMasterPubkey: Writable<string> = writable('');