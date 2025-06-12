<script>
    let { children, onClick, disabled } = $props();
</script>
<button
    onclick={onClick}
    class="border border-dark p-1 active:bg-dark active:text-white text-xs disabled:bg-gray-500 disabled:text-gray-300"
    disabled={disabled}
    >
    {@render children?.()}
</button>