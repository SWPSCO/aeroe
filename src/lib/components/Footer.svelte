<script lang="ts">
  import { getVersion } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { updater, node } from '$lib/services/tauri';
  import { open } from '@tauri-apps/plugin-dialog';

  let width = $state(0)
  let collapsed = $derived(width < 800)
  let version: string | null = $state(null)
  let updateInfo: { hasUpdate: boolean; updateVersion: string | null } = $state({
      hasUpdate: false,
      updateVersion: null,
  })
  let progress: { downloaded: number; total: number } = $state({
      downloaded: 0,
      total: 0,
  })
  let downloading = $state(false)
  
  // Separate node type and connection status
  let nodeType: 'onboard' | 'external' = $state('onboard')
  let isConnected = $state(false)
  let externalPath: string = $state('')
  let showExternalDialog = $state(false)
  let isConnecting = $state(false)

  // Derived status text
  const nodeStatusText = $derived(() => {
    const type = nodeType === 'onboard' ? 'Onboard' : 'External';
    const status = isConnected ? 'connected' : 'disconnected';
    return `${type} node ${status}`;
  });

  const updateApp = async () => {
      downloading = true;
      await updater.downloadAndInstallUpdate();
  }

  const selectExternalPath = async () => {
    try {
        const selected = await open({
            directory: false,
            multiple: false,
            filters: [{
                name: 'All files',
                extensions: ['*']
            }]
        });
        
        if (selected) {
            externalPath = selected;
        }
    } catch (error) {
        console.error('Failed to select file:', error);
    }
  }

  const connectExternal = async () => {
    console.log('connectExternal called with path:', externalPath.trim());
    
    if (!externalPath.trim()) {
        console.log('No path provided, returning early');
        return;
    }
    
    console.log('Setting isConnecting to true');
    isConnecting = true;
    
    try {
        console.log('About to call node.connectExternal...');
        const result = await node.connectExternal(externalPath.trim());
        console.log('connectExternal result:', result);
        
        console.log('About to call node.getStatus...');
        
        // Check the backend state after connection
        const status = await node.getStatus();
        console.log('getStatus after connect:', status);
        
        if (status.success && status.data) {
            nodeType = status.data.mode === 'external' ? 'external' : 'onboard';
            isConnected = status.data.connected;
            
            console.log('Updated frontend state - nodeType:', nodeType, 'isConnected:', isConnected);
            
            if (isConnected) {
                console.log('Connection successful, closing dialog');
                showExternalDialog = false;
            } else {
                console.log('Connection failed, keeping dialog open');
            }
        }
    } catch (error) {
        console.error('Error in connectExternal:', error);
    } finally {
        console.log('Setting isConnecting to false');
        isConnecting = false;
    }
}

  const startLocal = async () => {
      isConnecting = true;
      try {
          const result = await node.startMaster();
          console.log('startMaster result:', result);
          const status = await node.getStatus();
          console.log('getStatus after start:', status);
          
          if (status.success && status.data) {
              nodeType = status.data.mode === 'external' ? 'external' : 'onboard';
              isConnected = status.data.connected;
          }
      } catch (error) {
          console.error('Failed to start local node:', error);
      } finally {
          isConnecting = false;
      }
  }

  const disconnect = async () => {
      try {
          if (nodeType === 'onboard') {
              await node.stopMaster();
          }
          // Check backend state after disconnect
          const status = await node.getStatus();
          if (status.success && status.data) {
              nodeType = status.data.mode === 'external' ? 'external' : 'onboard';
              isConnected = status.data.connected;
          }
      } catch (error) {
          console.error('Failed to disconnect:', error);
      }
  }

  onMount(async () => {
      version = await getVersion() || "unknown"
      const updateListener = await listen<{ hasUpdate: boolean; updateVersion: string | null }>('update', (event) => {
          updateInfo = event.payload;
      });
      const updateDownloadedListener = await listen<{ downloaded: number; total: number }>('update_downloaded', (event) => {
          progress = event.payload;
      });
  })
</script>

<div bind:clientWidth={width} class="h-[42px] px-4 bg-dark flex items-center gap-4">
  <!-- Node Status -->
  <div class="flex items-center gap-2">
      <!-- Status Indicator -->
      <div class="w-2 h-2 rounded-full {isConnected ? 'bg-green-500' : 'bg-red-500'} animate-pulse"></div>
      
      <!-- Status Text -->
      <span class="text-light font-title text-xs">
          {nodeStatusText()}
      </span>
      
      <!-- Connection Controls -->
      {#if !isConnected}
          <button 
              class="text-light hover:text-highlight-orange font-title text-xs border border-light hover:border-highlight-orange px-1"
              onclick={startLocal}
              disabled={isConnecting}
          >
              {isConnecting ? '...' : 'start'}
          </button>
      {:else}
          <button 
              class="text-light hover:text-red-400 font-title text-xs border border-light hover:border-red-400 px-1"
              onclick={disconnect}
          >
              stop
          </button>
      {/if}
      
      <!-- External Connection Button -->
      <button 
          class="text-light hover:text-blue-400 font-title text-xs flex items-center gap-1 {isConnected && nodeType === 'external' ? 'text-blue-400' : ''}"
          onclick={() => showExternalDialog = true}
          disabled={false}
          title="Connect to external node"
      >
          <!-- Link/External Icon -->
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
          </svg>
      </button>
  </div>

  <!-- Aeroe Version -->
  {#if !collapsed}
  <div class="flex gap-2 justify-center items-center">
      <div class="h-[24px] px-1 font-title text-sm text-light border border-light">v{version}</div>
      {#if updateInfo.hasUpdate}
          {#if downloading}
              <div class="text-center px-1 font-title text-xs text-highlight-orange">
                  downloading update: {(progress.downloaded / 1024 / 1024).toFixed(2)} / {(progress.total / 1024 / 1024).toFixed(2)} MB
              </div>
          {:else}
              <div class="text-center px-1 font-title text-xs text-highlight-orange">
                  update available: v{updateInfo.updateVersion}
                  <button class="text-highlight-orange" onclick={updateApp}>[<span class="underline">download and install</span>]
                  </button>
              </div>
          {/if}
      {/if}
  </div>
  {/if}
  
  <div class="flex-1"></div>
  
  <div class="flex gap-{collapsed ? "1" : "4"} text-light font-title text-xs {collapsed ? "flex-col" :""}">
      <div><a class="text-highlight-orange" href="https://swps.io/terms-of-use" target="_blank">[terms]</a></div>
      <div><a class="text-highlight-orange" href="https://swps.io/privacy-policies" target="_blank">[privacy]</a></div>
  </div>
</div>

<!-- External Connection Dialog -->
{#if showExternalDialog}
<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
  <div class="bg-white p-6 rounded shadow-lg max-w-md w-full mx-4">
      <h3 class="text-lg font-title mb-4">Connect to external node socket</h3>
      
      <div class="flex flex-col gap-3">
          <div class="flex gap-2">
              <input 
                  type="text" 
                  bind:value={externalPath}
                  placeholder="/path/to/nockchain.sock"
                  class="flex-1 px-3 py-2 border border-gray-300 text-sm font-mono"
              />
              <button 
                  onclick={selectExternalPath}
                  class="px-3 py-2 bg-gray-100 hover:bg-gray-200 border border-gray-300 text-sm"
              >
                  Browse
              </button>
          </div>
          
          <div class="flex gap-2 justify-end">
              <button 
                  onclick={() => showExternalDialog = false}
                  class="px-4 py-2 text-gray-600 hover:text-gray-800"
                  disabled={isConnecting}
              >
                  Cancel
              </button>
              <button 
                  onclick={connectExternal}
                  disabled={!externalPath.trim() || isConnecting}
                  class="px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed"
              >
                  {isConnecting ? 'Connecting...' : 'Connect'}
              </button>
          </div>
      </div>
  </div>
</div>
{/if}