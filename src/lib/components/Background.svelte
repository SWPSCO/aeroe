<script>
	// import { onMount } from 'svelte';
	// onMount(() => {
	// 	// select layout element
	// 	const layout = document.getElementById('layout');
	// 	// set background image
	// 	layout.style.backgroundImage = 'url(/dot-bg.png)';
	// });
	$: format = "w-dvh h-dvh flex bg-cover";
</script>
<!-- <div id="layout" class="{format} bg-light dark:bg-dark bg-[url('/dot-bg.png')] dark:bg-[url('/dot-bg-dark.png')]"><slot /></div> -->
<div id="layout" class="{format} bg-light dark:bg-dark"><slot /></div>


<!-- <div class="w-dvh h-dvh flex bg-light dark:bg-dark bgpic dark:bgpic"><slot /></div>
<style>
	.bgpic {
		background-image: url('/dot-bg.png');
	}
	.dark\:bgpic {
		background-image: url('/dot-bg-dark.png');
	}
</style> -->
