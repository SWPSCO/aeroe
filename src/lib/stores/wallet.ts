import { writable } from 'svelte/store';
import { wallet as walletService, type WalletBalance } from '$lib/services';

export interface WalletState {
  status: 'locked' | 'loading' | 'loaded' | 'error';
  balance: WalletBalance | null;
  masterPubkey: string | null;
  error: string | null;
  // This could be expanded to include transaction history, etc.
}

function createWalletStore() {
  const { subscribe, update } = writable<WalletState>({
    status: 'locked',
    balance: null,
    masterPubkey: null,
    error: null,
  });

  async function fetchWalletData(walletName: string) {
    update(s => ({ ...s, status: 'loading', error: null }));

    // Use our new, type-safe service layer
    const balanceResult = await walletService.balance(walletName);
    const pubkeyResult = await walletService.masterPubkey(walletName);

    if (balanceResult.success && pubkeyResult.success) {
      const balanceData = balanceResult.data;
      const pubkeyData = pubkeyResult.data;

      if (balanceData && pubkeyData) {
        update(s => ({
          ...s,
          status: 'loaded',
          balance: balanceData,
          masterPubkey: pubkeyData,
        }));
      } else {
        update(s => ({
          ...s,
          status: 'error',
          error: 'Backend returned success but data was missing.',
        }));
      }
    } else {
      // Consolidate errors from the backend calls
      const balanceError = balanceResult.success ? null : JSON.stringify(balanceResult.error);
      const pubkeyError = pubkeyResult.success ? null : JSON.stringify(pubkeyResult.error);
      const error = [balanceError, pubkeyError].filter(e => e).join(', ');

      update(s => ({
        ...s,
        status: 'error',
        error: `Failed to load wallet data: ${error}`,
      }));
    }
  }

  function lock() {
    update(s => ({
        ...s,
        status: 'locked',
        balance: null,
        masterPubkey: null,
        error: null,
    }));
  }

  return {
    subscribe,
    fetchWalletData,
    lock,
  };
}

export const walletStore = createWalletStore(); 