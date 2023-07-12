
<script lang="ts">
  import githubIcon from '../../assets/github.svg';
  import Button from '../components/Button.svelte';
  import Email from '../icons/Email.svelte';
  import Input from '../components/Input.svelte';
  import EnableIcon from '../icons/Enable.svelte';
  import Login from '../icons/Login.svelte';
  import Password from '../icons/Password.svelte';
  import Admin from '../icons/Admin.svelte';
  import { push } from 'svelte-spa-router';

  const loginData = {
    email: '',
    password: '',
  };

  const registerData = {
    email: '',
    password: '',
    confirmPassword: '',
  };

  let erro = '';

  const firstTime = true;

</script>

<main>
  <div class="title">
    <EnableIcon />
    <h1>Enable</h1>
  </div>

  {#if !firstTime}
    <div class="form">
      <Input bind:value={loginData.email} placeholder="Email" type="email">
        <Email />
      </Input>
      <Input bind:value={loginData.password} placeholder="Password" type="password">
        <Password />
      </Input>
      <Button text="Login">
        <Login />
      </Button>
    </div>
  {:else}
    <div class="form">
      <p>Hello! Since this is your first time here, you need to create an admin account.</p>
      <Input bind:value={registerData.email} placeholder="Email" type="email" error={erro}>
        <Email />
      </Input>
      <Input bind:value={registerData.password} placeholder="Password" type="password" error={erro}>
        <Password />
      </Input>
      <Input bind:value={registerData.confirmPassword} placeholder="Confirm password" type="password" help="Use at least 8 characters" error={erro}>
        <Password />
      </Input>
      <Button text="Register" on:click={() => {
            erro = "Email zaralhado";
        }}>
        <Admin />
      </Button>
    </div>
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
<style>
  main {
    margin-top: 9rem;
  }
  .title {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }
  .form {
    display: flex;
    flex-direction: column;
    padding-bottom: 1rem;
    gap: 1rem;
    width: 100%;
    max-width: 420px;
  }

  .form > p {
    font-size: 14px;
    color: var(--main-content);
  }
  .links {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-top: 2em;
  }

  .links > a {
    padding: 1em;
    width: .5rem;
    height: .5rem;
    border-radius: 8px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

</style>