<script>
    let { children, onClick, disabled } = $props();
</script>
<button
    onclick={onClick}
    class="border border-dark p-1 active:bg-dark active:text-white text-xs"
    disabled={disabled}
    >
    {@render children?.()}
</button>