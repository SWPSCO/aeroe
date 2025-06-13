<script lang="ts">
	import PageLayout from '$lib/components/shared/PageLayout.svelte';
    import Button from '$lib/components/shared/Button.svelte';
</script>

<PageLayout title="Explorer">
    <div class="text-center p-8">
		<p class="text-gray-500">Blockchain explorer functionality will be implemented here.</p>
	</div>

    {#snippet actions()}
        <Button variant="secondary">Refresh</Button>
    {/snippet}
</PageLayout>