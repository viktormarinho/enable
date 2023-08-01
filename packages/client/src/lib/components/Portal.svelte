
<script>
  import { onDestroy, onMount } from "svelte";
    let ref, portal;

    onMount(() => {
        portal = document.createElement('div');
        portal.className = 'portal';
        document.body.appendChild(portal);
        portal.appendChild(ref);
    });

    onDestroy(() => {
        document.body.removeChild(portal);
    });
</script>

<div class="portal-clone">
    <div bind:this={ref} class="portal-inner">
        <slot></slot>
    </div>
</div>

<style>
    .portal-clone { display: none }
</style>