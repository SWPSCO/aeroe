<script lang="ts">
	import { loginStore } from '$lib/stores/login';
    import Button from '$lib/components/shared/Button.svelte';

	let password = '';
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if $loginStore.state === 'pending'}
		<div class="animate-pulse text-2xl font-title mb-4">Unlocking Vault...</div>
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
				onkeydown={(e) => e.key === 'Enter' && loginStore.login(password)}
			/>
			{#if $loginStore.state === 'error'}
				<p class="text-red-500 text-center">{$loginStore.error}</p>
			{/if}
		</div>
		<Button onclick={() => loginStore.login(password)}>
			Unlock
		</Button>
	{/if}
</div> 