import { writable, get } from 'svelte/store';
import { vault, wallet as walletService } from '$lib/services';
import { mainStore } from './main';
import { sessionStore } from './session';

export type WizardStep =
  | 'createPassword'
  | 'chooseAction'
  | 'createWallet'
  | 'importWallet'
  | 'finished';

interface WelcomeState {
  step: WizardStep;
  password?: string;
  seedPhrase?: string[];
  error?: string;
}

function createWelcomeStore() {
  const store = writable<WelcomeState>({
    step: 'createPassword',
  });
  const { subscribe, set, update } = store;

  async function submitPassword(password: string) {
    update(s => ({ ...s, password: password, error: undefined }));
    
    // Create the vault with the new password
    const result = await vault.create(password);

    if(result.success) {
        set({ step: 'chooseAction', password: password });
    } else {
        update(s => ({ ...s, error: 'Failed to create vault.' }));
    }
  }

  async function generateSeedPhrase() {
    update(s => ({ ...s, error: undefined }));
    const result = await walletService.keygen();
    if (result.success && result.data) {
      update(s => ({ ...s, seedPhrase: result.data, step: 'createWallet' }));
    } else {
      update(s => ({ ...s, error: 'Failed to generate seed phrase.' }));
    }
  }

  async function createWallet(walletName: string) {
    update(s => ({ ...s, error: undefined }));
    const currentState = get(store);
    const phrase = currentState.seedPhrase;

    if (!phrase) {
        update(s => ({ ...s, error: 'Seed phrase not found.' }));
        return;
    }

    const createResult = await walletService.create(walletName, phrase);
    console.log('createResult', createResult);

    if(createResult.success) {
        // This is the crucial missing step. We must load the wallet to make it active.
        console.log('about to load wallet', walletName);
        let loadResult;
        try {
          loadResult = await walletService.load(walletName);
          console.log('loadResult', loadResult);
        } catch (e) {
          console.error('load threw', e);
          update(s => ({ ...s, error: `Wallet created, but load threw: ${e}` }));
          return;
        }
        if (loadResult.success) {
            completeOnboarding(walletName);
        } else {
            update(s => ({ ...s, error: `Wallet created, but failed to load: ${JSON.stringify(loadResult.error)}` }));
        }
    } else {
        update(s => ({ ...s, error: `Failed to create wallet: ${JSON.stringify(createResult.error)}` }));
    }
  }

  async function importWallet(walletName: string, seedPhrase: string[]) {
    update(s => ({ ...s, error: undefined }));
    
    if(seedPhrase.some(word => word.trim() === '') || seedPhrase.length !== 24) {
        update(s => ({ ...s, error: 'Please enter all 24 words.'}));
        return;
    }

    const createResult = await walletService.create(walletName, seedPhrase);
    console.log('import createResult', createResult);

    if(createResult.success) {
        // This is the crucial missing step. We must load the wallet to make it active.
        let loadResult;
        try {
          loadResult = await walletService.load(walletName);
          console.log('import loadResult', loadResult);
        } catch (e) {
          console.error('import load threw', e);
          update(s => ({ ...s, error: `Wallet imported, but load threw: ${e}` }));
          return;
        }
        if (loadResult.success) {
            completeOnboarding(walletName);
        } else {
            update(s => ({ ...s, error: `Wallet imported, but failed to load: ${JSON.stringify(loadResult.error)}` }));
        }
    } else {
        update(s => ({ ...s, error: `Failed to import wallet: ${JSON.stringify(createResult.error)}` }));
    }
  }

  function chooseCreate() {
    generateSeedPhrase();
  }

  function chooseImport() {
    update(s => ({ ...s, step: 'importWallet' }));
  }
  
  function completeOnboarding(walletName: string) {
    // update session wallets list optimistically
    sessionStore.addWallet(walletName);
    mainStore.authenticate(walletName); // redirect
    set({ step: 'finished' });
  }

  // Reset the wizard for adding an extra wallet when user is already authenticated
  function resetForAdd() {
    set({ step: 'chooseAction', error: undefined });
  }

  return {
    subscribe,
    submitPassword,
    chooseCreate,
    chooseImport,
    createWallet,
    importWallet,
    completeOnboarding,
    resetForAdd,
  };
}

export const welcomeStore = createWelcomeStore(); 