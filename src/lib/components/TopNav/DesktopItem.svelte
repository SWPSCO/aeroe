<script>
    import { goto } from '$app/navigation';
    export let active = false;
    export let route = '';
    $: text = "text-2xl font-title uppercase font-medium"
    $: color = active ? "text-highlight-orange" : "text-dark"
    $: hover = "hover:text-highlight-orange"
    $: border = active ? "border-b-4 border-highlight-orange" : "border-b-4 border-light"
</script>
<button
    class="h-[66px] border-b-4 border-dark {text} {color} {hover} {border}"
    on:click={() => goto(route)}>
    <slot />
</button>

<style>
    button {
        text-align: right;
        font-style: normal;
        letter-spacing: 1.96px;
        text-transform: uppercase;
        margin-bottom: -4px;
    }
</style>