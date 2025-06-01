import { writable } from 'svelte/store';

export const previousPage = writable('/');

export const walletLoaded = writable(false);
export const walletBalance = writable(0);
export const walletMasterPubkey = writable('');