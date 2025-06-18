<script lang="ts">
  import { node } from "$lib/services/tauri";
  import Button from "./Button.svelte";
  import { onMount } from "svelte";

  let masterStatus: any = $state(undefined);
  let nodePeekStatus: any = $state(undefined);
  let externalSocketPath: string = $state("");
  let nodeMode: 'local' | 'external' | 'disconnected' = $state('disconnected');
  let isLoading: boolean = $state(false);

  const updateNodeStatus = async () => {
      try {
          const status = await node.getStatus();
          if (status.success && status.data) {
              nodeMode = status.data.mode;
          }
      } catch (error) {
          console.error('Failed to get node status:', error);
      }
  };

  onMount(() => {
      updateNodeStatus();
      const interval = setInterval(updateNodeStatus, 2000);
      return () => clearInterval(interval);
  });

  const startMaster = async () => {
      isLoading = true;
      try {
          const res = await node.startMaster();
          masterStatus = res;
          await updateNodeStatus();
      } catch (error) {
          masterStatus = error;
      } finally {
          isLoading = false;
      }
  }

  const stopMaster = async () => {
      isLoading = true;
      try {
          const res = await node.stopMaster();
          masterStatus = res;
          await updateNodeStatus();
      } catch (error) {
          masterStatus = error;
      } finally {
          isLoading = false;
      }
  }

  const connectExternal = async () => {
      if (!externalSocketPath.trim()) {
          masterStatus = "Socket path cannot be empty";
          return;
      }
      
      isLoading = true;
      try {
          const res = await node.connectExternal(externalSocketPath.trim());
          masterStatus = res;
          nodeMode = 'external';
      } catch (error) {
          masterStatus = error;
      } finally {
          isLoading = false;
      }
  }

  const nodePeek = async (command: string) => {
      nodePeekStatus = undefined;
      try {
          const res = await node.peek(command);
          nodePeekStatus = res;
      } catch (error) {
          nodePeekStatus = error;
      }
  }

  const getStatusColor = () => {
      switch (nodeMode) {
          case 'local': return 'text-green-500';
          case 'external': return 'text-blue-500';
          default: return 'text-gray-500';
      }
  }
</script>

<div class="flex flex-col gap-4 border-2 border-dark p-4">
  <div class="flex flex-col gap-2 text-xs font-title">
      <div class="flex items-center gap-2">
          <div>Node Status:</div>
          <div class={`font-bold ${getStatusColor()}`}>
              {nodeMode.toUpperCase()}
          </div>
      </div>
      
      <!-- Local Node Controls -->
      <div class="flex gap-4 items-center">
          <div>Local node:</div>
          <div class="flex gap-2">
              <Button 
                  onClick={startMaster} 
                  disabled={isLoading || nodeMode === 'local' || nodeMode === 'external'}
              >
                  {isLoading ? 'Starting...' : 'Start'}
              </Button>
              <Button 
                  onClick={stopMaster} 
                  disabled={isLoading || nodeMode !== 'local'}
              >
                  {isLoading ? 'Stopping...' : 'Stop'}
              </Button>
          </div>
      </div>

      <!-- External Node Controls -->
      <div class="flex gap-4 items-center">
          <div>External node:</div>
          <div class="flex gap-2 items-center">
              <input 
                  type="text" 
                  bind:value={externalSocketPath}
                  placeholder="/path/to/socket"
                  class="px-2 py-1 border border-gray-300 text-xs"
                  disabled={nodeMode === 'local' || nodeMode === 'external'}
              />
              <Button 
                  onClick={connectExternal} 
                  disabled={isLoading || nodeMode === 'local' || nodeMode === 'external' || !externalSocketPath.trim()}
              >
                  {isLoading ? 'Connecting...' : 'Connect'}
              </Button>
          </div>
      </div>

      <!-- Status Display -->
      {#if masterStatus !== undefined}
          <div class="text-xs p-2 bg-gray-100 rounded">
              Status: {JSON.stringify(masterStatus)}
          </div>
      {/if}
  </div>

  <!-- Peek Controls -->
  <div class="flex gap-4 text-xs font-title items-center">
      <div>Peeks:</div>
      <div class="flex gap-2">
          <Button 
              onClick={() => nodePeek("height")} 
              disabled={nodeMode === 'disconnected'}
          >
              Height
          </Button>
          <Button 
              onClick={() => nodePeek("heavy-summary")} 
              disabled={nodeMode === 'disconnected'}
          >
              Heavy Summary
          </Button>
          <Button 
              onClick={() => nodePeek("transactions")} 
              disabled={nodeMode === 'disconnected'}
          >
              Transactions
          </Button>
      </div>
      {#if nodePeekStatus !== undefined}
          <div class="text-xs p-2 bg-gray-100 rounded">
              {JSON.stringify(nodePeekStatus)}
          </div>
      {/if}
  </div>
</div>