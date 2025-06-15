import type { BackendResponse, AeroeStatus, WalletBalance, NockchainTxMeta } from './tauri';

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
    transactions: Record<string, NockchainTxMeta>;
} = {
    termsAccepted: false,
    privacyAccepted: false,
    vaultExists: false,
    wallets: [],
    activeWallet: null,
    balances: {},
    transactions: {},
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
        mockState.balances[walletName] = 0; // Set initial balance to 0 for new wallets
        return mockInvoke<void>(undefined);
    },
    load: (walletName: string) => {
        mockState.activeWallet = walletName;
        return mockInvoke<void>(undefined);
    },
    masterPubkey: (walletName: string) => mockInvoke<string>(`mock-pubkey-for-${walletName}`),
    balance: (walletName: string) => {
        const amount = mockState.balances[walletName] ?? 123.45;
        return mockInvoke<WalletBalance>({ coin: 'Nock', amount });
    },
    getHistory: (walletName: string) => mockInvoke<any>({ transactions: [] }),
    listDrafts: (walletName: string) => mockInvoke<any>({ drafts: [] }),
    createDraft: (walletName: string) => mockInvoke<any>({ draftId: 'mock-draft-123' }),
    sendTransaction: (walletName: string, draftId: string) => mockInvoke<any>({ txid: 'mock-txid-abc' }),
    createTx: (
        walletName: string,
        transactions: { recipient: string, amount: number }[],
        fee: number,
    ) => {
        const draftId = crypto.randomUUID();
        const newTx: NockchainTxMeta = {
            draftId,
            transactions,
            fee,
            createdAt: new Date().toISOString(),
            signedAt: null,
            broadcastedAt: null,
            status: 'draft',
        };
        mockState.transactions[draftId] = newTx;
        console.log(`Mock transaction created for ${walletName}:`, newTx);
        return mockInvoke<NockchainTxMeta>(newTx);
    },
    signTx: (walletName: string, draftId: string) => {
        const tx = mockState.transactions[draftId];
        if (tx) {
            tx.status = 'signed';
            tx.signedAt = new Date().toISOString();
            console.log(`Mock transaction signed for ${walletName}:`, tx);
            return mockInvoke<NockchainTxMeta>(tx);
        }
        return mockInvoke<NockchainTxMeta>({} as NockchainTxMeta, false);
    },
    sendTx: (walletName: string, draftId: string) => {
        const tx = mockState.transactions[draftId];
        if (tx) {
            tx.status = 'pending';
            tx.broadcastedAt = new Date().toISOString();
            console.log(`Mock transaction sent for ${walletName}:`, tx);
            return mockInvoke<NockchainTxMeta>(tx);
        }
        return mockInvoke<NockchainTxMeta>({} as NockchainTxMeta, false);
    },
    listUnsentTxs: (walletName: string) => {
        console.log(`Listing unsent mock transactions for ${walletName}:`, mockState.transactions);
        return mockInvoke<{ [draftId: string]: NockchainTxMeta }>(mockState.transactions);
    }
};

export const node = {
    startMaster: () => mockInvoke<void>(undefined),
    stopMaster: () => mockInvoke<void>(undefined),
    peek: (command: string) => mockInvoke<any>({ result: `mock result for ${command}` }),
}; 