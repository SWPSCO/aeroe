<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
</script>
<div class="flex flex-col items-center justify-center gap-4">
    <h1 class="text-4xl font-title">Welcome to <span class="text-highlight-orange">AEROE</span></h1>
    <h4 class="text-md font-title">Brought to you by the <span class="text-highlight-orange">Southwestern Pool Supply Company</span></h4>
</div>
<div class="flex gap-4">
    <button class="w-[240px] bg-dark text-white py-4" onclick={() => dispatch("import")}>Import Wallet</button>
    <button
        class="w-[240px] bg-dark text-white py-4"
        onclick={() => dispatch("create")}
    >Create Wallet</button>
</div>