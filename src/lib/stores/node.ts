import { writable, get } from 'svelte/store';
import { node as nodeService } from '$lib/services/tauri';

export interface NodeState {
  connected: boolean;
  mode: 'local' | 'external' | 'disconnected';
  blockHeight: number | null;
  connecting: boolean;
  error: string | null;
  heightPolling: boolean;
}

function createNodeStore() {
  const store = writable<NodeState>({
    connected: false,
    mode: 'disconnected',
    blockHeight: null,
    connecting: false,
    error: null,
    heightPolling: false,
  });
  const { subscribe, update } = store;

  let heightInterval: ReturnType<typeof setInterval> | null = null;

  async function getStatus() {
    const result = await nodeService.getStatus();
    if (result.success && result.data) {
      update(s => ({
        ...s,
        connected: result.data?.connected || false,
        mode: result.data?.mode === 'external' ? 'external' : result.data?.mode === 'local' ? 'local' : 'disconnected',
        error: null,
      }));
      
      if (result.data.connected) {
        startHeightPolling();
      } else {
        stopHeightPolling();
        update(s => ({ ...s, blockHeight: null }));
      }
    } else {
      update(s => ({ ...s, error: `Failed to get node status: ${JSON.stringify(result.error)}` }));
    }
  }

  async function startLocal() {
    update(s => ({ ...s, connecting: true, error: null }));
    
    try {
      const result = await nodeService.startMaster();
      if (result.success) {
        await getStatus();
      } else {
        update(s => ({ 
          ...s, 
          connecting: false,
          error: `Failed to start local node: ${JSON.stringify(result.error)}` 
        }));
      }
    } catch (error) {
      update(s => ({ 
        ...s, 
        connecting: false,
        error: `Failed to start local node: ${error}` 
      }));
    } finally {
      update(s => ({ ...s, connecting: false }));
    }
  }

  async function connectExternal(socketPath: string) {
    update(s => ({ ...s, connecting: true, error: null }));
    
    try {
      const result = await nodeService.connectExternal(socketPath);
      if (result.success) {
        await getStatus();
      } else {
        update(s => ({ 
          ...s, 
          connecting: false,
          error: `Failed to connect to external node: ${JSON.stringify(result.error)}` 
        }));
      }
    } catch (error) {
      update(s => ({ 
        ...s, 
        connecting: false,
        error: `Failed to connect to external node: ${error}` 
      }));
    } finally {
      update(s => ({ ...s, connecting: false }));
    }
  }

  async function disconnect() {
    const currentState = get(store);
    
    try {
      if (currentState.mode === 'local') {
        await nodeService.stopMaster();
      } else if (currentState.mode === 'external') {
        await nodeService.disconnectExternal();
      }
      await getStatus();
    } catch (error) {
      update(s => ({ ...s, error: `Failed to disconnect: ${error}` }));
    }
  }

  async function pollBlockHeight() {
    const currentState = get(store);
    if (!currentState.connected || currentState.heightPolling) return;
    
    update(s => ({ ...s, heightPolling: true }));
    
    try {
      const result = await nodeService.peek('height');
      if (result.success && result.data !== undefined) {
        update(s => ({ ...s, blockHeight: result.data, error: null }));
      }
    } catch (error) {
      console.error('Failed to get block height:', error);
    } finally {
      update(s => ({ ...s, heightPolling: false }));
    }
  }

  function startHeightPolling() {
    if (heightInterval) clearInterval(heightInterval);
    
    pollBlockHeight();
    heightInterval = setInterval(pollBlockHeight, 60000);
  }

  function stopHeightPolling() {
    if (heightInterval) {
      clearInterval(heightInterval);
      heightInterval = null;
    }
  }

  function setError(error: string) {
    update(s => ({ ...s, error }));
  }

  function clearError() {
    update(s => ({ ...s, error: null }));
  }

  function cleanup() {
    stopHeightPolling();
  }

  return {
    subscribe,
    getStatus,
    startLocal,
    connectExternal,
    disconnect,
    pollBlockHeight,
    setError,
    clearError,
    cleanup,
  };
}

export const nodeStore = createNodeStore();