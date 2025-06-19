import { writable } from 'svelte/store';
import { mainStore } from './main';
import { sessionStore } from './session';
import { vault, aeroe } from '$lib/services';

type State = 'idle' | 'pending' | 'error';

type LoginStore = {
  state: State;
  error: string | null;
};

function createLoginStore() {
  const { subscribe, update } = writable<LoginStore>({
    state: 'idle',
    error: null,
  });

  async function login(password: string) {
    update((store) => ({ ...store, state: 'pending', error: null }));

    const result = await vault.load(password.trim());

    if (result.success) {
      const statusRes = await aeroe.status();

      if (statusRes.success && statusRes.data && statusRes.data.wallets.length > 0) {
        // Set wallets in session store before navigating
        sessionStore.setWallets(statusRes.data.wallets || []);
        // Redirect to wallet selection page instead of auto-loading
        mainStore.navigateToWalletSelection();
      } else {
        update((store) => ({
          ...store,
          state: 'error',
          error: `Vault unlocked, but no wallets found or status failed. Error: ${JSON.stringify(statusRes.error)}`,
        }));
      }
    } else {
      update((store) => ({
        ...store,
        state: 'error',
        error: `Failed to unlock vault: ${JSON.stringify(result.error)}`,
      }));
    }
  }

  return {
    subscribe,
    login,
  };
}

export const loginStore = createLoginStore();
