<script lang="ts">
  import githubIcon from "../../assets/github.svg";
  import EnableIcon from "../icons/Enable.svelte";
  import { onMount } from "svelte";
  import LoginForm from "../components/LoginForm.svelte";
  import RegisterForm from "../components/RegisterForm.svelte";

  let firstTime = true;
  onMount(async () => {
    const res = await fetch("/api/auth/first-time");
    const { isFirstTime } = await res.json();
    firstTime = isFirstTime;
  });
</script>

<div class="page">
  <main>
    <div class="title">
      <EnableIcon />
      <h1>Enable</h1>
    </div>

    {#if !firstTime}
      <LoginForm />
    {:else}
      <RegisterForm />
    {/if}

    <p class="links">
      <a
        href="https://github.com/viktormarinho/enable"
        target="_blank"
        rel="noreferrer"
        class="hover"
      >
        <img src={githubIcon} alt="Github logo" />
      </a>
    </p>
  </main>
</div>

<style>
  main {
    margin-top: 9rem;
  }
  .page {
    width: 1280px;
    margin: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    text-align: center;
  }
  .title {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }
  .links {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-top: 2em;
  }

  .links > a {
    padding: 1em;
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 8px;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
