import { writable, get } from 'svelte/store';
import { wallet as walletService } from '$lib/services';
import { aeroe } from '$lib/services';
import type { WalletBalance, NockchainTxMeta } from '$lib/services/tauri';
import { sessionStore } from '$lib/stores/session';

export interface WalletState {
  status: 'locked' | 'loading' | 'loaded' | 'error';
  balance: WalletBalance | null;
  masterPubkey: string | null;
  transactions: { [draftId: string]: NockchainTxMeta };
  error: string | null;
  loadedWalletName: string | null;
  fetching: boolean;
}

function createWalletStore() {
  const store = writable<WalletState>({
    status: 'locked',
    balance: null,
    masterPubkey: null,
    transactions: {},
    error: null,
    loadedWalletName: null,
    fetching: false,
  });
  const { subscribe, update } = store;

  const RETRY_DELAY = 500; // ms
  const MAX_WAIT_MS = 30000;

  async function fetchWalletData(walletName: string): Promise<void> {
    // prevent duplicate fetches for wallet already loaded
    const currentState = get(store);
    if (currentState.status === 'loaded' && currentState.loadedWalletName === walletName) return;

    // Determine if this is the first time we are loading any wallet
    const firstLoad = get(store).loadedWalletName === null;

    update(s => ({
      ...s,
      status: firstLoad ? 'loading' : 'loaded',
      fetching: true,
      error: null,
    }));

    const start = Date.now();

    while (true) {
      const [balanceRes, pubkeyRes, txsRes] = await Promise.all([
        walletService.balance(walletName),
        walletService.masterPubkey(walletName),
        walletService.listUnsentTxs(walletName)
      ]);

      if (balanceRes.success && balanceRes.data) {
        update(s => ({
          ...s,
          status: 'loaded',
          fetching: false,
          balance: balanceRes.data as WalletBalance,
          masterPubkey: pubkeyRes.success ? pubkeyRes.data ?? null : null,
          transactions: txsRes.success ? txsRes.data ?? {} : {},
          error: null,
          loadedWalletName: walletName,
        }));

        // Ensure session wallet list is up-to-date (handles cold start)
        const statusRes = await aeroe.status();
        if (statusRes.success && statusRes.data) {
          sessionStore.setWallets(statusRes.data.wallets || []);
        }
        return;
      }

      if (Date.now() - start > MAX_WAIT_MS) {
        const err = JSON.stringify(balanceRes.error ?? 'Balance not ready');
        update(s => ({
          ...s,
          status: firstLoad ? 'error' : 'loaded',
          fetching: false,
          error: `Failed to load wallet balance: ${err}`,
        }));
        return;
      }

      await new Promise(r => setTimeout(r, RETRY_DELAY));
    }
  }

  async function createTransaction(recipient: string, amount: number, fee: number, walletName: string) {
    const result = await walletService.createTx(walletName, [{ recipient, amount }], fee);

    if (result.success && result.data) {
      // Add the new transaction to our local state
      const newTx = result.data;
      if (newTx) {
        update(s => {
          const newTxs = { ...s.transactions, [newTx.draftId]: newTx };
          return { ...s, transactions: newTxs, error: null };
        });
      }
    } else {
      update(s => ({ ...s, error: `Failed to create transaction: ${JSON.stringify(result.error)}` }));
    }
  }

  async function signTransaction(draftId: string, walletName: string) {
    const result = await walletService.signTx(walletName, draftId);
    if (result.success && result.data) {
      update(s => {
        const newTxs = { ...s.transactions, [draftId]: result.data! };
        return { ...s, transactions: newTxs, error: null };
      });
    } else {
      update(s => ({ ...s, error: `Failed to sign transaction: ${JSON.stringify(result.error)}` }));
    }
  }

  async function sendTransaction(draftId: string, walletName: string) {
    const result = await walletService.sendTx(walletName, draftId);
    if (result.success && result.data) {
      update(s => {
        const newTxs = { ...s.transactions, [draftId]: result.data! };
        return { ...s, transactions: newTxs, error: null };
      });
    } else {
      update(s => ({ ...s, error: `Failed to send transaction: ${JSON.stringify(result.error)}` }));
    }
  }

  function setError(error: string) {
    update(s => ({ ...s, error }));
  }

  function clearError() {
    update(s => ({ ...s, error: null }));
  }

  function lock() {
    update(s => ({
        ...s,
        status: 'locked',
        balance: null,
        masterPubkey: null,
        transactions: {},
        error: null,
        loadedWalletName: null,
        fetching: false,
    }));
  }

  return {
    subscribe,
    fetchWalletData,
    // Helper used by UI when switching wallets: ensures backend has the wallet loaded before fetching data
    async loadAndFetch(walletName: string) {
      try {
        await walletService.load(walletName);
        // refresh list of wallets from backend so menu is accurate
        const statusRes = await aeroe.status();
        if (statusRes.success && statusRes.data) {
          const current = get(sessionStore).wallets;
          const merged = Array.from(new Set([...(current ?? []), ...((statusRes.data.wallets) ?? [])]));
          sessionStore.setWallets(merged);
        }
      } catch (_) {
        // Even if load fails, still attempt to fetch to surface error via balance
      }
      await fetchWalletData(walletName);
    },
    createTransaction,
    signTransaction,
    sendTransaction,
    setError,
    clearError,
    lock,
  };
}

export const walletStore = createWalletStore(); 