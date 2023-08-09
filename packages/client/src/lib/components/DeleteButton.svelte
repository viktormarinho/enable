<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Trash from "../icons/Trash.svelte";
  import { clickOutside } from "../util/clickOutside";
  import Button from "./Button.svelte";
  import Portal from "./Portal.svelte";

  const dispatch = createEventDispatcher();

  export let label = "";
  let open = false;
</script>

<button class="trash" on:click={() => (open = true)}>
  <span>
    <Trash />
  </span>
</button>
{#if open}
  <Portal>
    <div
      use:clickOutside
      on:click_outside={() => (open = false)}
      class="hidden"
      class:open
    >
      <p>
        <span class="">Are you sure you want to delete {label}?</span>
        <span class="red label">This operation cannot be undone</span>.
      </p>
      <Button on:click={() => {
        open = false;
        dispatch('confirm');
      }} text="Confirm" />
      <button on:click={() => (open = false)} class="cancel">Cancel</button>
    </div>
  </Portal>
{/if}

<style>
  .open {
    display: flex;
    flex-direction: column;
    pointer-events: all;
    gap: 8px;
    text-align: center;
    position: absolute;
    z-index: 10;
    width: 20vw;
    height: fit-content;
    border-radius: 8px;
    top: 45vh;
    left: 40vw;
    border: 1px solid #aaa;
    background-color: #fff;
    box-shadow: var(--shadow);
    padding: 8px;
  }
  .trash {
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
  .trash:hover {
    background-color: rgb(235, 235, 235);
  }
  .trash > span {
    color: var(--main-content);
    background-color: none;
  }
  .cancel {
    border-radius: 0.5rem;
    cursor: pointer;
    padding: 1rem 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    color: var(--main-content);
    background-color: whitesmoke;
    border: none;
    font-size: 16px;
    font-weight: bold;
  }
  .cancel:hover {
    background-color: rgb(230, 230, 230);
  }
  .label {
    font-weight: bold;
  }
  .red {
    color: red;
  }
</style>
