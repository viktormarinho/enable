<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { clickOutside } from "../util/clickOutside";

  let email = "";
  let open = false;

  onMount(async () => {
    const res = await fetch("/api/auth/me");
    const { email: _email } = await res.json();
    email = _email;
  });
</script>

<div>
  <button
    class="user-button"
    class:modal-open={open}
    on:click={() => (open = !open)}
  >
    {(email.at(0) ?? " ").toUpperCase()}
  </button>
  <div style="position: relative;">
    {#key open}
      <div
        class="hidden"
        class:open
        in:fly={{ y: 5 }}
        use:clickOutside
        on:click_outside={() => (open = false)}
      >
        <span>{email}</span>
        <a href="#/admin/settings" class="link hover">Settings</a>
        <a href="/api/auth/logout" class="link hover">Logout</a>
      </div>
    {/key}
  </div>
</div>

<style>
  .user-button {
    border-radius: 9999px;
    background-color: whitesmoke;
    color: var(--sec-content);
    font-weight: 600;
    width: 3rem;
    height: 3rem;
    font-size: 18px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    border: none;
    padding: 0;
  }
  .user-button:hover {
    background-color: rgb(235, 235, 235);
  }
  .user-button:focus {
    outline: none;
  }
  .open {
    color: var(--sec-content);
    display: block;
    position: absolute;
    border-radius: 8px;
    top: 8px;
    background-color: white;
    z-index: 10;
    display: flex;
    box-shadow: var(--shadow);
    flex-direction: column;
    border: 1px solid #e6e6e6;
    padding: 8px;
    gap: 2px;
  }
  .open > * {
    padding: 6px;
  }
  .link {
    border-radius: 8px;
    cursor: pointer;
    color: var(--sec-content);
    font-weight: 500;
    border: none;
    outline: none;
    text-decoration: none;
  }
</style>
