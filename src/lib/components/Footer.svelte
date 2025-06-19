<script lang="ts">
  import { getVersion } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { updater, node } from '$lib/services/tauri';
  import { nodeStore } from '$lib/stores/node';
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
  
  let nodeType = $derived($nodeStore.mode === 'external' ? 'external' : 'onboard')
  let isConnected = $derived($nodeStore.connected)
  let isConnecting = $derived($nodeStore.connecting)
  let blockHeight = $derived($nodeStore.blockHeight)
  let externalPath: string = $state('')
  let showExternalDialog = $state(false)
  let showDropdown = $state(false)

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

  const startLocal = () => nodeStore.startLocal()
  const connectExternal = () => nodeStore.connectExternal(externalPath.trim())
  const disconnect = () => nodeStore.disconnect()

  onMount(() => {
      const init = async () => {
          version = await getVersion() || "unknown"
          const updateListener = await listen<{ hasUpdate: boolean; updateVersion: string | null }>('update', (event) => {
              updateInfo = event.payload;
          });
          const updateDownloadedListener = await listen<{ downloaded: number; total: number }>('update_downloaded', (event) => {
              progress = event.payload;
          });
          
          const status = await nodeStore.getStatus()
      };
      
      init();

      // Click outside to close dropdown
      const handleClickOutside = (event: MouseEvent) => {
          if (showDropdown && !(event.target as Element)?.closest('.relative')) {
              showDropdown = false;
          }
      };
      document.addEventListener('click', handleClickOutside);
      
      return () => {
          document.removeEventListener('click', handleClickOutside);
          nodeStore.cleanup();
      };
  })
</script>

<div bind:clientWidth={width} class="h-[42px] px-4 bg-dark flex items-center gap-4">
  <!-- Node Status with Dropdown -->
  <div class="flex items-center gap-2">
      <!-- Status Indicator -->
      <div class="w-2 h-2 rounded-full {isConnected ? 'bg-green-500' : 'bg-red-500'} animate-pulse"></div>
      
      <!-- Node Type Dropdown -->
      <div class="relative">
          <button 
              class="h-[24px] text-light ont-title hover:text-highlight-orange text-sm border border-light hover:border-highlight-orange px-2 py-2 flex items-center whitespace-nowrap overflow-hidden"
              onclick={() => showDropdown = !showDropdown}
              disabled={isConnecting}
          >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
                  <rect x="6" y="6" width="12" height="12" rx="1"/>
              </svg>
              {nodeType === 'onboard' ? 'Local' : 'External'}
          </button>
          
          {#if showDropdown}
          <div class="absolute bottom-full left-0 mb-1 bg-white border border-gray-300 shadow-lg z-50 min-w-[80px]">
              <button 
                  class="w-full px-3 py-2 text-left text-xs hover:bg-gray-100 text-black {nodeType === 'onboard' ? 'bg-gray-50' : ''}"
                  onclick={() => { showDropdown = false; if (!isConnected) startLocal(); }}
                  disabled={isConnecting}
              >
                  Local
              </button>
              <button 
                  class="w-full px-3 py-2 text-left text-xs hover:bg-gray-100 text-black {nodeType === 'external' ? 'bg-gray-50' : ''}"
                  onclick={() => { showDropdown = false; showExternalDialog = true; }}
              >
                  External
              </button>
          </div>
          {/if}
      </div>
      
      <!-- Connection Status & Control -->
      {#if isConnected}
          <button 
              class="h-[24px] text-light font-title hover:text-highlight-orange text-sm border border-light hover:text-highlight-orange px-2 py-2 items-center flex"
              onclick={disconnect}
              title="Stop/Disconnect node"
          >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
                  <rect x="6" y="6" width="12" height="12" rx="1"/>
              </svg>
              Stop
          </button>
      <!-- Block Height -->
      {#if  blockHeight !== null}
          <div class="flex items-center gap-1">
              <span class="text-light font-title text-sm">Tip:</span>
              <span class="text-highlight-orange font-title text-sm">{blockHeight}</span>
          </div>
      {/if}
      {:else}
          <button 
              class="text-light hover:text-green-400 font-title text-xs border border-light hover:border-green-400 px-2 py-1 flex items-center gap-1 whitespace-nowrap overflow-hidden"
              onclick={() => { if (nodeType === 'onboard') startLocal(); else showExternalDialog = true; }}
              disabled={isConnecting}
          >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
                  <polygon points="5,3 19,12 5,21"/>
              </svg>
              {isConnecting ? 'connecting...' : 'Start'}
          </button>
      {/if}
  </div>

  <!-- Aeroe Version -->
  {#if !collapsed}
  <div class="flex gap-2 justify-center items-center">
      <div class="h-[24px] px-1 font-title text-sm text-light border border-light whitespace-nowrap overflow-hidden">v{version}</div>
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