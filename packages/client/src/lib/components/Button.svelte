<script lang="ts">
  import Spinner from "../icons/Spinner.svelte";

  export let text: string;
  export let loading: boolean = false;
</script>

<button on:click>
  {#if loading}
    <Spinner />
  {:else}
    <slot />
  {/if}
  <span>{text}</span>
</button>

<style>
  button {
    border-radius: 0.5rem;
    cursor: pointer;
    padding: 1rem 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    color: white;
    background-color: var(--main-content);
    border: none;
    font-size: 16px;
    font-weight: bold;
  }

  button:hover {
    opacity: 0.9;
  }
</style>
