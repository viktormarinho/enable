
<script lang="ts">
  import githubIcon from '../../assets/github.svg';
  import Button from '../components/Button.svelte';
  import Email from '../icons/Email.svelte';
  import Input from '../components/Input.svelte';
  import EnableIcon from '../icons/Enable.svelte';
  import Login from '../icons/Login.svelte';
  import Password from '../icons/Password.svelte';
  import Admin from '../icons/Admin.svelte';
  import { onMount } from 'svelte';
  import { push } from 'svelte-spa-router';
  import { errors } from '../errors';
  import ErrorMessage from '../components/ErrorMessage.svelte';
  import { getNotificationsContext } from 'svelte-notifications';

  const { addNotification } = getNotificationsContext();

  const loginData = {
    email: '',
    password: '',
  };
  let loginErrors = errors.make(loginData);

  const registerData = {
    email: '',
    password: '',
    passwordConfirm: '',
  };
  let registerErrors = errors.make(registerData);

  let firstTime = true;
  onMount(async () => {
    const res = await fetch('/api/auth/first-time');
    const { isFirstTime } = await res.json();
    firstTime = isFirstTime;
  });

  let loading = false;

  const login = async () => {
    loading = true;
    loginErrors = errors.reset(loginErrors);
    const res = await fetch('/api/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(loginData),
    });

    if (res.status === 200) {
      push('/admin');
    } else {
      loginErrors.email = 'Email ou senha incorretos';
      loginErrors.password = 'Email ou senha incorretos';
    }
    loading = false;
  };

  const register = async () => {
    loading = true;
    registerErrors = errors.reset(registerErrors);
    const res = await fetch('/api/auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(registerData),
    });

    if (res.ok) {
      push('/admin');
      addNotification({
        type: 'success',
        position: 'top-right',
        removeAfter: 3000,
        text: 'Account registered with success.',
        id: 'account-created'
      });
    } else { 
      registerErrors = errors.apply(registerErrors, await res.json());
    }
    loading = false;
  };
</script>

<main>
  <div class="title">
    <EnableIcon />
    <h1>Enable</h1>
  </div>

  {#if !firstTime}
    <div class="form">
      <p>Welcome back to the enable admin dashboard.</p>
      <Input bind:value={loginData.email} placeholder="Email" type="email" error={loginErrors.email}>
        <Email />
      </Input>
      <Input bind:value={loginData.password} placeholder="Password" type="password" error={loginErrors.password}>
        <Password />
      </Input>
      {#if loginErrors.general}
        <ErrorMessage message={loginErrors.general} />
      {/if}
      <Button loading={loading} text="Login" on:click={login}>
        <Login />
      </Button>
    </div>
  {:else}
    <div class="form">
      <p>Hello! Since this is your first time here, you need to create an admin account.</p>
      <Input bind:value={registerData.email} placeholder="Email" type="email" error={registerErrors.email}>
        <Email />
      </Input>
      <Input bind:value={registerData.password} placeholder="Password" type="password" error={registerErrors.password}>
        <Password />
      </Input>
      <Input bind:value={registerData.passwordConfirm} placeholder="Confirm password" type="password" help="Use at least 8 characters" error={registerErrors.passwordConfirm}>
        <Password />
      </Input>
      {#if registerErrors.general}
        <ErrorMessage message={registerErrors.general} />
      {/if}
      <Button loading={loading} text="Register" on:click={register}>
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
    width: 420px;
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