<script lang=ts>
  import { push } from "svelte-spa-router";
  import { errors } from "../errors";
  import { getNotificationsContext } from "svelte-notifications";
  import Button from "./Button.svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import Input from "./Input.svelte";
  import AdminIcon from "../icons/Admin.svelte";
  import PasswordIcon from "../icons/Password.svelte";
  import EmailIcon from "../icons/Email.svelte";

  const { addNotification } = getNotificationsContext();

  let loading = false;
  const registerData = {
    email: '',
    password: '',
    passwordConfirm: '',
  };
  let registerErrors = errors.make(registerData);

  const register = async (e: Event) => {
    e.preventDefault();
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

<form class="form" on:submit={register}>
  <p>
    Hello! Since this is your first time here, you need to create an admin
    account.
  </p>
  <Input
    bind:value={registerData.email}
    placeholder="Email"
    type="email"
    error={registerErrors.email}
  >
    <EmailIcon />
  </Input>
  <Input
    bind:value={registerData.password}
    placeholder="Password"
    type="password"
    error={registerErrors.password}
  >
    <PasswordIcon />
  </Input>
  <Input
    bind:value={registerData.passwordConfirm}
    placeholder="Confirm password"
    type="password"
    help="Use at least 8 characters"
    error={registerErrors.passwordConfirm}
  >
    <PasswordIcon />
  </Input>
  {#if registerErrors.general}
    <ErrorMessage message={registerErrors.general} />
  {/if}
  <Button {loading} text="Register" on:click={register}>
    <AdminIcon />
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