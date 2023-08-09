<script lang="ts">
  import CopyIcon from "../icons/Copy.svelte";
  import CheckIcon from "../icons/Check.svelte";

  export let text = "";

  let copied = false;

  function copy() {
    navigator.clipboard.writeText(text);
    copied = true;
    setTimeout(() => (copied = false), 1500);
  }
</script>

<button class:copied={copied} on:click={copy}>
    <span>
        {#if !copied}
            <CopyIcon />
        {:else}
            <CheckIcon />
        {/if}
    </span>
</button>

<style>
  button {
    border-radius: 8px;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: white;
    border: none;
    cursor: pointer;
    height: 2.5rem;
    width: 2.5rem;
  }
  button:hover {
    background-color: rgb(235,235,235);
  }
  button > span {
    color: var(--main-content);
    background-color: none;
  }
  .copied {
    color: green;
    background-color: rgb(176, 255, 176);
  }
  .copied:hover {
    background-color: rgb(176, 255, 176);
  }
</style>
