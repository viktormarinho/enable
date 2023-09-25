<script lang=ts>
  import { errors } from "../errors";
  import EnableIcon from "../icons/Enable.svelte";
import { clickOutside } from "../util/clickOutside";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Portal from "./Portal.svelte";

    export let projectId: string;
    export let projectName: string;
    
    let createFeatureData = {
        name: '',
        project_id: projectId
    };
    let createFeatureErrors = errors.make(createFeatureData);
    
    let displayName = '';
    $: {
        if (createFeatureData.name.length > 0) {
            displayName = `${projectName}_${createFeatureData.name}`.toLowerCase().split(" ").join("_");
        } else {
            displayName = '';
        }
    } 

    let open = false;
    let loading = false;

    async function createFeature(e: Event) {
        e.preventDefault();
        loading = true;
        const res = await fetch('/api/features', {
            method: "POST",
            body: JSON.stringify(createFeatureData),
            headers: {
                'Content-Type': "application/json"
            }
        });

        if (!res.ok) {
            createFeatureErrors = errors.apply(createFeatureErrors, await res.json());
            loading = false;
            return;
        }

        location.reload();
    }
</script>

<button class="btn-secondary" on:click={() => (open = true)}>
  <span> Create feature + </span>
</button>
{#if open}
    <Portal>
        <div use:clickOutside on:click_outside={() => (open = false)} class="hidden" class:open={open}>
            <form class="modal-content" on:submit={createFeature}>
                <span>New feature</span>
                <Input type="text" bind:value={createFeatureData.name} placeholder="Feature name" error={''}>
                    <EnableIcon width='32' height='32'/>
                </Input>
                <p>Feature identifier: <strong>{displayName}</strong></p>
                <p class="help">This is a unique identifier for this feature across all your projects.</p>
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
</style>