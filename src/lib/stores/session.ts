import { writable } from 'svelte/store';

export interface SessionState {
  wallets: string[];
  activeWalletName: string | null;
  // This could be expanded in the future with user profiles, etc.
}

function createSessionStore() {
  const { subscribe, set, update } = writable<SessionState>({
    wallets: [],
    activeWalletName: null,
  });

  return {
    subscribe,
    setWallets: (names: string[]) => update(s => ({ ...s, wallets: names })),
    addWallet: (name: string) => update(s => ({ ...s, wallets: [...s.wallets, name] })),
    setActiveWallet: (walletName: string) => update(s => ({ ...s, activeWalletName: walletName })),
    clearSession: () => set({ wallets: [], activeWalletName: null }),
  };
}

export const sessionStore = createSessionStore(); 