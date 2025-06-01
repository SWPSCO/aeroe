<script>
	import '../../app.css';
	import Background from '$lib/components/Background.svelte';
	import MainArea from '$lib/components/MainArea.svelte';
	import TopNav from '$lib/components/TopNav.svelte';
	import ContentArea from '$lib/components/ContentArea.svelte';
	import FeatureArea from '$lib/components/FeatureArea.svelte';
	import Footer from '$lib/components/Footer.svelte';
	let { children } = $props();
</script>

<Background>
	<MainArea>
		<TopNav />
		<ContentArea>
			<FeatureArea>
				{@render children()}
			</FeatureArea>
		</ContentArea>
		<Footer />
	</MainArea>
</Background>