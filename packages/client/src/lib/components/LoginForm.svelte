<script lang=ts>
  import { push } from "svelte-spa-router";
  import { errors } from "../errors";
  import Input from "./Input.svelte";
  import EmailIcon from "../icons/Email.svelte";
  import LoginIcon from "../icons/Login.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import Button from "./Button.svelte";
  import PasswordIcon from "../icons/Password.svelte";

  let loading = false;

  const loginData = {
    email: '',
    password: '',
  };
  let loginErrors = errors.make(loginData);

  const login = async (e: Event) => {
    e.preventDefault();
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
</script>

<form class="form" on:submit={login}>
  <p>Welcome back to the enable admin dashboard.</p>
  <Input
    bind:value={loginData.email}
    placeholder="Email"
    type="email"
    error={loginErrors.email}
  >
    <EmailIcon />
  </Input>
  <Input
    bind:value={loginData.password}
    placeholder="Password"
    type="password"
    error={loginErrors.password}
  >
    <PasswordIcon />
  </Input>
  {#if loginErrors.general}
    <ErrorMessage message={loginErrors.general} />
  {/if}
  <Button {loading} text="Login" on:click={login}>
    <LoginIcon />
  </Button>
</form>

<style>
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
</style>