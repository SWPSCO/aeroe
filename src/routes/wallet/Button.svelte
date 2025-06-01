<script>
    export let color = "bg-light"
    export let textColor = "text-dark"
</script>
<button class="border-2 border-dark {color} h-[52px] w-[104px] uppercase font-title text-sm {textColor}">
    <slot />
</button>