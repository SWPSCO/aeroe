import { writable } from 'svelte/store';

export interface SessionState {
  activeWalletName: string | null;
  // This could be expanded in the future with user profiles, etc.
}

function createSessionStore() {
  const { subscribe, set, update } = writable<SessionState>({
    activeWalletName: null,
  });

  return {
    subscribe,
    setActiveWallet: (walletName: string) => update(s => ({ ...s, activeWalletName: walletName })),
    clearSession: () => set({ activeWalletName: null }),
  };
}

export const sessionStore = createSessionStore(); 