import { invoke, type InvokeArgs } from '@tauri-apps/api/core';

// Generic response type for all commands
export interface BackendResponse<T> {
  success: boolean;
  data?: T;
  error?: unknown;
}

// Specific data structures returned from Rust
export interface WalletBalance {
  coin: string;
  amount: number;
}

export interface AeroeStatus {
  is_first_run: boolean;
  vaultExists: boolean;
  vaultLoaded: boolean;
  wallets: string[];
  activeWallet: string | null;
}


async function handleInvoke<T>(command: string, args?: InvokeArgs): Promise<BackendResponse<T>> {
    try {
        const data = await (args ? invoke(command, args) : invoke(command));
        return { success: true, data: data as T };
    } catch (error) {
        return { success: false, error: error };
    }
}

async function notImplemented<T>(): Promise<BackendResponse<T>> {
    return { success: false, error: "Not implemented" };
}


export const terms = {
    isTermsAccepted: () => handleInvoke<boolean>('terms_of_use_is_accepted'),
    isPrivacyAccepted: () => handleInvoke<boolean>('privacy_policy_is_accepted'),
    setTermsAccepted: () => handleInvoke<void>('accept_terms_of_use'),
    setPrivacyAccepted: () => handleInvoke<void>('accept_privacy_policy'),
}

export const updater = {
    downloadAndInstallUpdate: () => handleInvoke<void>('download_and_install_update'),
}

export const aeroe = {
    status: () => handleInvoke<AeroeStatus>('aeroe_status'),
}

export const vault = {
    create: (password: string) => handleInvoke<void>('vault_create', { password }),
    load: (password: string) => handleInvoke<void>('vault_load', { password }),
}

export const wallet = {
    keygen: () => handleInvoke<string[]>('keygen'),
    create: (walletName: string, seedphrase: string[]) => handleInvoke<void>('wallet_create', { walletName, seedphrase }),
    load: (walletName: string) => handleInvoke<void>('wallet_load', { walletName }),
    masterPubkey: (walletName: string) => handleInvoke<string>('master_pubkey', { walletName }),
    balance: (walletName: string) => handleInvoke<WalletBalance>('balance', { walletName }),
    getHistory: (walletName: string) => notImplemented<any>(),
    listDrafts: (walletName: string) => notImplemented<any>(),
    createDraft: (walletName: string) => notImplemented<any>(),
    sendTransaction: (walletName: string, draftId: string) => notImplemented<any>(),
}

export const node = {
    startMaster: () => handleInvoke<void>('node_start_master'),
    stopMaster: () => handleInvoke<void>('node_stop_master'),
    peek: (command: string) => handleInvoke<any>('node_peek', { command }),
} 