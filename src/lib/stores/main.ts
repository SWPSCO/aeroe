import { writable, get } from 'svelte/store';
import { sessionStore } from './session';
import { aeroe, terms } from '$lib/services';
import { goto } from '$app/navigation';

// Define the possible states
type StateName = 'booting' | 'onboarding' | 'unauthenticated' | 'authenticated' | 'error';

export interface MainStoreState {
  name: StateName;
  error?: string | null;
}

function createMainStore() {
  const store = writable<MainStoreState>({ name: 'booting' });
  const { subscribe } = store;

  async function boot() {

    
    if (get(store).name !== 'booting') {
      return;
    }

    store.set({ name: 'booting' });
    const termsRes = await terms.isTermsAccepted();
    const privacyRes = await terms.isPrivacyAccepted();

    if (!termsRes.success || !privacyRes.success || !termsRes.data || !privacyRes.data) {
        store.set({ name: 'onboarding' });
        goto('/'); // Navigate to the landing page for the onboarding flow
        return;
    }

    // If terms are accepted, proceed to check vault status
    const statusRes = await aeroe.status();
    if (statusRes.success && statusRes.data) {
        sessionStore.setWallets(statusRes.data.wallets || []);
        if (statusRes.data.vaultExists) {
            store.set({ name: 'unauthenticated' });
            goto('/login');
        } else {
            store.set({ name: 'unauthenticated' }); // Or a 'welcoming' state if we want more granular control
            goto('/welcome');
        }
    } else {
        store.set({ name: 'error', error: `Failed to get application status: ${JSON.stringify(statusRes.error)}` });
    }
  }

  async function authenticate(walletName: string) {
    sessionStore.setActiveWallet(walletName);
    store.set({ name: 'authenticated' });
    goto('/wallet');
  }

  function unauthenticate() {
    sessionStore.clearSession();
    store.set({ name: 'unauthenticated' });
    goto('/login');
  }

  function completeOnboarding() {
    store.set({ name: 'unauthenticated' });
    goto('/welcome');
  }

  function navigateToWalletSelection() {
    store.set({ name: 'unauthenticated' });
    goto('/select-wallet');
  }

  // Initial boot sequence is now removed from here
  // boot();

  return {
    subscribe,
    boot,
    authenticate,
    unauthenticate,
    completeOnboarding,
    navigateToWalletSelection,
  };
}

export const mainStore = createMainStore(); 