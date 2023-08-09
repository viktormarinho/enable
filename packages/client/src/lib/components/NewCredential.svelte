<script lang="ts">
  import { getNotificationsContext } from "svelte-notifications";
  import Tag from "../icons/Tag.svelte";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import { createEventDispatcher } from "svelte";

  const { addNotification } = getNotificationsContext();
  const dispatch = createEventDispatcher();

  let newCredentialName = "";
  let newCredentialNameError = "";
  let loading = false;

  async function createCredential(e: Event) {
    e.preventDefault();
    newCredentialNameError = "";
    loading = true;
    if (newCredentialName.length < 3) {
      newCredentialNameError = "Please pick a name with at least 3 characters";
      loading = false;
      return;
    }

    const res = await fetch("/api/credentials", {
      method: "POST",
      body: JSON.stringify({ name: newCredentialName }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (res.ok) {
      addNotification({
        type: "success",
        position: "top-right",
        removeAfter: 3000,
        text: "Credential created with success.",
        id: "credential-created",
      });
      dispatch('created');
      newCredentialName = '';
    } else {
      addNotification({
        type: "error",
        position: "top-right",
        removeAfter: 3000,
        text: "Could not create credential.",
        id: "credential-created-error",
      });
    }

    loading = false;
  }
</script>

<form on:submit={createCredential}>
  <Input
    bind:value={newCredentialName}
    error={newCredentialNameError}
    placeholder="Credential name"
  >
    <Tag />
  </Input>
  <Button text="Create" {loading}>
    <span>+</span>
  </Button>
</form>

<style>
  form {
    display: flex;
    gap: 8px;
  }
</style>
