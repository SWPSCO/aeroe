<script lang="ts">
	import { loginStore } from '$lib/stores/login';
	import { node } from "$lib/services/tauri";
    import Button from '$lib/components/shared/Button.svelte';

	let password = '';
	let showAdvanced = false;
	let externalNodePath = '';
	let isConnectingExternal = false;

  const handleLogin = async () => {
    const nodeStatus = await node.getStatus();
    
    if (!nodeStatus.data?.connected) {
        alert("Please connect to a node (local or external) before unlocking your wallet");
        return;
    }

    if (externalNodePath.trim()) {
        isConnectingExternal = true;
        try {
            await node.connectExternal(externalNodePath.trim());
            await loginStore.login(password);
        } catch (error) {
            console.error('Failed to connect to external node:', error);
        } finally {
            isConnectingExternal = false;
        }
    } else {
        await loginStore.login(password);
    }
  };
	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Enter') {
			handleLogin();
		}
	};
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if $loginStore.state === 'pending' || isConnectingExternal}
		<div class="animate-pulse text-2xl font-title">
			{isConnectingExternal ? 'Connecting to External Node...' : 'Unlocking Vault...'}
		</div>
		<div>
			<svg width="160" height="100" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
				<style>
					:root { --t: 2.4s }
					@keyframes jiggle {
						0%,20%,55%,100% { transform:rotate(0) }
						32%             { transform:rotate(-10deg) }
						44%             { transform:rotate(10deg) }
					}
					@keyframes pop {
						0%,10%,35%,100% { transform:translateY(0) }
						22%             { transform:translateY(-16px) }
						28%             { transform:translateY(-12px) }
					}
					@keyframes glow {
						0%,18%,50%,100% { stroke:#000 }
						34%             { stroke:#ff4a1a }
					}
					.body { fill:#000; rx:8; ry:8 }
					.shackle { stroke:#000; stroke-width:6; fill:none; stroke-linecap:round;
							  transform-origin:80px 35px; animation:pop var(--t) cubic-bezier(.3,1.4,.4,1) infinite }
					.key { fill:#ff4a1a; transform-origin:80px 72px; animation:jiggle var(--t) ease-in-out infinite }
					.shine { stroke:#000; stroke-width:3; stroke-linecap:round; opacity:.25;
							  animation:glow var(--t) ease-in-out infinite }
				</style>
				<line class="shine" x1="58" y1="44" x2="102" y2="44"/>
				<rect class="body" x="50" y="42" width="60" height="46"/>
				<path class="shackle" d="M60 44 v-18 a20 20 0 0 1 40 0 v18"/>
				<g class="key">
					<rect x="75" y="60" width="10" height="24" rx="2"/>
					<rect x="70" y="78" width="20" height="6"  rx="1"/>
				</g>
			</svg>
		</div>
	{:else}
		<h1 class="text-2xl font-title">Unlock Your Vault</h1>
		<p class="text-md font-title text-center max-w-md">
			Enter your local password to access your wallets.
		</p>
		
		<div class="flex flex-col gap-4 w-full max-w-sm">
			<input
				type="password"
				bind:value={password}
				class="text-lg font-title text-dark border border-dark p-3 text-center w-full focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
				placeholder="Password"
				aria-label="Password"
				onkeydown={handleKeydown}
			/>
			
			<!-- Advanced Options Toggle -->
			<button 
				class="text-sm text-gray-600 hover:text-gray-800 underline"
				onclick={() => showAdvanced = !showAdvanced}
			>
				{showAdvanced ? 'Hide' : 'Show'} Advanced Options
			</button>
			
			{#if showAdvanced}
				<div class="border border-gray-300 p-4 rounded bg-gray-50">
					<label class="block text-sm font-medium text-gray-700 mb-2">
						Node NPC socket path (optional)
					<input
						type="text"
						bind:value={externalNodePath}
						class="text-sm font-mono border border-gray-300 p-2 w-full focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
						placeholder="/path/to/nockchain/socket"
						aria-label="External node socket path"
					/>
          </label>
					<p class="text-xs text-gray-500 mt-1">
						Leave empty to start a local node automatically
					</p>
				</div>
			{/if}
			
			{#if $loginStore.state === 'error'}
				<p class="text-red-500 text-center">{$loginStore.error}</p>
			{/if}
		</div>
		
		<Button onclick={handleLogin}>
			Unlock
		</Button>
	{/if}
</div>