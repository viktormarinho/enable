<script>
  import EnableIcon from "../icons/Enable.svelte";
  import User from "./User.svelte";

</script>

<nav class="navbar">
    <a href="#/admin" class="logo">
        <EnableIcon width='32' height='32' />
        <h3>Enable</h3>
    </a>
    <User/>
</nav>

<style>
    .navbar {
        display: flex;
        width: 100vw;
        align-items: center;
        justify-content: space-around;
        padding: 1rem;
        box-sizing: border-box;
    }
    
    .logo {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        text-decoration: none;
        color: var(--main-content);
    }
</style>