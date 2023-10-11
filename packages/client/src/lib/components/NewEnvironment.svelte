
<script lang=ts>
  import { errors } from "../errors";
  import EnvironmentIcon from "../icons/Environment.svelte";
  import { clickOutside } from "../util/clickOutside";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Portal from "./Portal.svelte";

    export let projectId: string;
    export let projectName: string;
    
    let createEnvironmentData = {
        name: '',
        project_id: projectId
    };
    let createEnvironmentErrors = errors.make(createEnvironmentData);
    
    let displayName = '';
    $: {
        if (createEnvironmentData.name.length > 0) {
            displayName = `${projectName}_${createEnvironmentData.name}`.toLowerCase().split(" ").join("_");
        } else {
            displayName = '';
        }
    } 

    let open = false;
    let loading = false;

    async function createEnvironment(e: Event) {
        e.preventDefault();
        loading = true;
        const res = await fetch('/api/environment', {
            method: "POST",
            body: JSON.stringify(createEnvironmentData),
            headers: {
                'Content-Type': "application/json"
            }
        });

        if (!res.ok) {
            createEnvironmentErrors = errors.apply(createEnvironmentErrors, await res.json());
            loading = false;
            return;
        }

        location.reload();
    }
</script>

<button class="btn-secondary" on:click={() => (open = true)}>
  <span> Environment </span>
</button>
{#if open}
    <Portal>
        <div use:clickOutside on:click_outside={() => (open = false)} class="hidden" class:open={open}>
            <form class="modal-content" on:submit={createEnvironment}>
                <span>New environment</span>
                <Input maxLength="20" type="text" bind:value={createEnvironmentData.name} placeholder="Environment name" error={createEnvironmentErrors.name}>
                    <EnvironmentIcon width='32' height='32'/>
                </Input>
                <p class="help">Tip: Try to choose a short name.</p>
                <Button text="Create" loading={loading} />
            </form>
        </div>
    </Portal>
{/if}

<style>
  .open {
    display: block;
    pointer-events: all;
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
  .modal-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .modal-content > * {
    margin: 0;
  }
  .modal-content > span {
    font-size: 24px;
    text-align: center;
    margin-bottom: 8px;
    font-weight: 500;
  }
  .modal-content > p {
    font-size: 12px;
  }

  strong {
    overflow-wrap: break-word;
  }
</style>