import type { BackendResponse, AeroeStatus, WalletBalance } from './tauri';

// Helper function to simulate async backend calls
async function mockInvoke<T>(data: T, success = true, delay = 250): Promise<BackendResponse<T>> {
  return new Promise(resolve => {
    setTimeout(() => {
      if (success) {
        resolve({ success: true, data });
      } else {
        resolve({ success: false, error: 'Mocked backend error' });
      }
    }, delay);
  });
}

// --- Mock Data and State ---
// You can change these values to test different scenarios
const mockState: {
    termsAccepted: boolean;
    privacyAccepted: boolean;
    vaultExists: boolean;
    wallets: string[];
    activeWallet: string | null;
    balances: Record<string, number>;
} = {
    termsAccepted: false,
    privacyAccepted: false,
    vaultExists: false,
    wallets: [],
    activeWallet: null,
    balances: {},
};


export const terms = {
    isTermsAccepted: () => mockInvoke<boolean>(mockState.termsAccepted),
    isPrivacyAccepted: () => mockInvoke<boolean>(mockState.privacyAccepted),
    setTermsAccepted: () => {
        mockState.termsAccepted = true;
        return mockInvoke<void>(undefined);
    },
    setPrivacyAccepted: () => {
        mockState.privacyAccepted = true;
        return mockInvoke<void>(undefined);
    },
};

export const updater = {
    downloadAndInstallUpdate: () => mockInvoke<void>(undefined),
};

export const aeroe = {
    status: () => mockInvoke<AeroeStatus>({
        is_first_run: !mockState.vaultExists,
        vaultExists: mockState.vaultExists,
        vaultLoaded: false,
        wallets: mockState.wallets,
        activeWallet: mockState.activeWallet,
    }),
};

export const vault = {
    create: (password: string) => {
        console.log(`Mock vault created with password: ${password}`);
        mockState.vaultExists = true;
        return mockInvoke<void>(undefined);
    },
    load: (password: string) => {
        console.log(`Mock vault loaded with password: ${password}`);
        return mockInvoke<void>(undefined);
    },
};

export const wallet = {
    keygen: () => mockInvoke<string[]>(Array.from({ length: 24 }, (_, i) => `word${i + 1}`)),
    create: (walletName: string, seedphrase: string[]) => {
        console.log(`Mock wallet created: ${walletName} with seed`, seedphrase);
        mockState.wallets.push(walletName);
        return mockInvoke<void>(undefined);
    },
    load: (walletName: string) => {
        mockState.activeWallet = walletName;
        return mockInvoke<void>(undefined);
    },
    masterPubkey: (walletName: string) => mockInvoke<string>(`mock-pubkey-for-${walletName}`),
    balance: (walletName: string) => mockInvoke<WalletBalance>({ coin: 'Nock', amount: 123.45 }),
    getHistory: (walletName: string) => mockInvoke<any>({ transactions: [] }),
    listDrafts: (walletName: string) => mockInvoke<any>({ drafts: [] }),
    createDraft: (walletName: string) => mockInvoke<any>({ draftId: 'mock-draft-123' }),
    sendTransaction: (walletName: string, draftId: string) => mockInvoke<any>({ txid: 'mock-txid-abc' }),
};

export const node = {
    startMaster: () => mockInvoke<void>(undefined),
    stopMaster: () => mockInvoke<void>(undefined),
    peek: (command: string) => mockInvoke<any>({ result: `mock result for ${command}` }),
}; 