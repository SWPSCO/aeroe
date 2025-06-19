<script lang="ts">
  export let open: boolean = false;
  export let close: () => void = () => {};
  export let small: boolean = false;

  $: console.log('Modal open prop', open);
</script>

{#if open}
  <!-- Backdrop --> 
  <div class="fixed inset-0 bg-black/50 z-50" on:click={close}></div>

  <!-- Clicks on darkened area (outside panel) close modal -->
  <div class="fixed inset-0 flex items-center justify-center z-50" on:click={close}>
    <div class="bg-light border-2 border-dark shadow-lg overflow-y-auto"
         class:w-[80vw]={!small}
         class:h-[80vh]={!small}
         class:max-w-[80vw]={!small}
         class:max-h-[80vh]={!small}
         class:p-6={small}
         on:click|stopPropagation>
      <slot />
    </div>
  </div>
{/if} 