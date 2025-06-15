import { writable } from 'svelte/store';
import { mainStore } from './main';
import { vault, aeroe, wallet } from '$lib/services';

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
        const walletToLoad = statusRes.data.wallets[0];
        const loadRes = await wallet.load(walletToLoad);

        if(loadRes.success) {
          mainStore.authenticate(walletToLoad);
        } else {
          update((store) => ({
            ...store,
            state: 'error',
            error: `Vault unlocked, but failed to load wallet '${walletToLoad}'. Error: ${JSON.stringify(loadRes.error)}`,
          }));
        }
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
