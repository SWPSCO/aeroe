<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<div class="flex flex-col gap-4 justify-center items-center h-full text-center">
	<h1 class="text-4xl font-title">Welcome to Nock</h1>
	<p class="font-body text-lg">Your gateway to the Nockchain.</p>

	<div class="mt-8 flex flex-col gap-4">
		<button
			class="w-[240px] bg-dark text-white p-4 font-title"
			on:click={() => dispatch('create')}
		>
			Create New Wallet
		</button>
		<button
			class="w-[240px] border-2 border-dark text-dark p-4 font-title"
			on:click={() => dispatch('import')}
		>
			Import Existing Wallet
		</button>
	</div>
</div>