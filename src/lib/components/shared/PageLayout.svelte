<script lang="ts">
    interface Props {
        title: string;
        actions?: any; // The 'any' type is used for renderable content in Svelte 5
        children: any;
    }
    const { title, actions, children }: Props = $props();
</script>

<div class="p-4 sm:p-8">
    <header class="flex justify-between items-center mb-4">
        <h1 class="text-2xl font-title text-dark">{title}</h1>
        <div class="actions flex gap-2">
            {#if actions}
                {@render actions()}
            {/if}
        </div>
    </header>

    <main class="content bg-white p-4 rounded-md shadow">
        {@render children()}
    </main>
</div> 