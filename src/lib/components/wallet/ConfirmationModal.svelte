<script lang="ts">
	export let recipient: string;
	export let amount: number;
	export let fee: number;
	export let onConfirm: () => void;
	export let onCancel: () => void;
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center">
	<div class="bg-white p-8 rounded-lg shadow-xl w-full max-w-md border-4 border-dark">
		<h2 class="font-title text-2xl text-center">Confirm Transaction</h2>

		<div class="mt-6 flex flex-col gap-4">
			<div>
				<p class="font-title text-sm text-gray-500">You are sending</p>
				<p class="text-3xl font-bold mt-1">{amount} Nock</p>
			</div>
			<div>
				<p class="font-title text-sm text-gray-500">To</p>
				<p class="font-mono text-sm break-all mt-1">{recipient}</p>
			</div>
			<div>
				<p class="font-title text-sm text-gray-500">Estimated Fee</p>
				<p class="font-mono text-sm mt-1">{fee} Nock</p>
			</div>
		</div>

		<div class="mt-8 flex gap-4">
			<button on:click={onCancel} class="flex-1 p-4 border-2 border-dark font-title">
				Cancel
			</button>
			<button on:click={onConfirm} class="flex-1 p-4 bg-dark text-white font-title">
				Confirm
			</button>
		</div>
	</div>
</div> 