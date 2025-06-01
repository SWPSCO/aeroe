<script>
	import '../../app.css';
	import Background from '$lib/components/Background.svelte';
	import MainArea from '$lib/components/MainArea.svelte';
	let { children } = $props();
</script>

<Background>
	<MainArea>
		{@render children()}
	</MainArea>
</Background>