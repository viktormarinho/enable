<script lang="ts">
  import { onMount } from "svelte";
  import NewCredential from "./NewCredential.svelte";
  import type { Credential } from "../types/credential";
  import CopyButton from "./CopyButton.svelte";
  import Password from "../icons/Password.svelte";
  import DeleteButton from "./DeleteButton.svelte";
  import { getNotificationsContext } from "svelte-notifications";

  const { addNotification } = getNotificationsContext();
  let credentials: Credential[] = [];

  const loadCredentials = async () => {
    const res = await fetch("/api/credentials");
    credentials = await res.json();
  };

  const deleteCredential = async (id: number) => {
    const res = await fetch(`/api/credentials/${id}`, {
      method: "DELETE",
    });

    if (res.ok) {
      addNotification({
        type: "success",
        position: "top-right",
        removeAfter: 3000,
        text: "Credential removed with success.",
        id: "credential-removed",
      });
      loadCredentials();
    } else {
      addNotification({
        type: "error",
        position: "top-right",
        removeAfter: 3000,
        text: "Could not remove credential.",
        id: "credential-removed",
      });
    }
  };

  onMount(loadCredentials);
</script>

<ul>
  {#each credentials as credential}
    <li>
      <Password />
      <span class="name">{credential.name}</span>
      <span class="pass">••••••••••••••••••••••••••••••••••••••••••</span>
      <CopyButton text={credential.token} />
      <DeleteButton
        on:confirm={() => deleteCredential(credential.id)}
        label={`the credential ${credential.name}`}
      />
    </li>
  {/each}
</ul>
<NewCredential on:created={loadCredentials} />

<style>
  ul {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 16px;
    color: var(--main-content);
    padding: 0;
  }
  li {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 8px;
    border-radius: 8px;
    width: 60%;
    font-size: 14px;
  }
  .name {
    font-weight: bold;
    padding: 8px;
    border-radius: 8px;
    color: var(--main-content);
  }
  .pass {
    background-color: whitesmoke;
    border-radius: 8px;
    padding: 8px;
    margin-left: auto;
  }
</style>
