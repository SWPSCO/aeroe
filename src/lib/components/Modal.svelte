<script lang="ts">
  export let open: boolean = false;
  export let close: () => void = () => {};

  $: console.log('Modal open prop', open);
</script>

{#if open}
  <!-- Backdrop --> 
  <div class="fixed inset-0 bg-black/50 z-50" on:click={close}></div>

  <!-- Panel -->
  <div class="fixed inset-0 flex items-center justify-center z-50" on:click|stopPropagation>
    <div class="bg-light border-2 border-dark w-[80vw] max-w-[80vw] h-[80vh] max-h-[80vh] shadow-lg overflow-y-auto">
      <slot />
    </div>
  </div>
{/if} 