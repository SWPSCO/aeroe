<script lang="ts">
	import { onMount } from 'svelte';
	import QRCode from 'qrcode';

	let { address } = $props<{ address: string }>();
	let canvas: HTMLCanvasElement;

	onMount(() => {
		if (canvas && address) {
			QRCode.toCanvas(canvas, address, {
				width: 256,
				margin: 2,
				errorCorrectionLevel: 'H'
			});
		}
	});
</script>

<canvas bind:this={canvas}></canvas> 