import { writable, get } from 'svelte/store';
import { wallet as walletService } from '$lib/services';
import type { WalletBalance, NockchainTxMeta } from '$lib/services/tauri';
import { sessionStore } from '$lib/stores/session';

export interface WalletState {
  status: 'locked' | 'loading' | 'loaded' | 'error';
  balance: WalletBalance | null;
  masterPubkey: string | null;
  transactions: { [draftId: string]: NockchainTxMeta };
  error: string | null;
  // This could be expanded to include transaction history, etc.
}

function createWalletStore() {
  const store = writable<WalletState>({
    status: 'locked',
    balance: null,
    masterPubkey: null,
    transactions: {},
    error: null,
  });
  const { subscribe, update } = store;

  async function fetchWalletData(walletName: string) {
    const currentState = get(store);
    // If the requested wallet is already loaded, don't re-fetch.
    if (currentState.status === 'loaded' && get(sessionStore).activeWalletName === walletName) {
      return;
    }

    update(s => ({ ...s, status: 'loading', error: null }));

    // Use our new, type-safe service layer
    const balanceResult = await walletService.balance(walletName);
    const pubkeyResult = await walletService.masterPubkey(walletName);
    const txsResult = await walletService.listUnsentTxs(walletName);

    if (balanceResult.success && pubkeyResult.success && txsResult.success) {
      const balanceData = balanceResult.data;
      const pubkeyData = pubkeyResult.data;
      const txsData = txsResult.data;

      if (balanceData && pubkeyData && txsData) {
        update(s => ({
          ...s,
          status: 'loaded',
          balance: balanceData,
          masterPubkey: pubkeyData,
          transactions: txsData,
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
      const txsError = txsResult.success ? null : JSON.stringify(txsResult.error);
      const error = [balanceError, pubkeyError, txsError].filter(e => e).join(', ');

      update(s => ({
        ...s,
        status: 'error',
        error: `Failed to load wallet data: ${error}`,
      }));
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
    }));
  }

  return {
    subscribe,
    fetchWalletData,
    createTransaction,
    signTransaction,
    sendTransaction,
    setError,
    clearError,
    lock,
  };
}

export const walletStore = createWalletStore(); 