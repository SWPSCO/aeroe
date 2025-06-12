<script lang="ts">
    type ButtonVariant = 'primary' | 'secondary';

    interface Props {
        variant?: ButtonVariant;
        disabled?: boolean;
        onclick?: (event: MouseEvent) => void;
        children: any;
    }

    const {
        variant = 'primary',
        disabled = false,
        onclick,
        children,
    }: Props = $props();

    const baseClasses = "py-4 px-8 font-title transition-colors duration-150 disabled:opacity-50";

    const variantClasses = {
        primary: "bg-dark text-white hover:bg-gray-700",
        secondary: "bg-gray-200 text-dark hover:bg-gray-300",
    };
</script>

<button
    class="{baseClasses} {variantClasses[variant]}"
    {disabled}
    on:click={onclick}
>
    {@render children()}
</button> 