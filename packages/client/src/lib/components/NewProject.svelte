<script lang="ts">
  import { push } from "svelte-spa-router";
  import { errors } from "../errors";
  import { clickOutside } from "../util/clickOutside";
  import Input from "./Input.svelte";
  import Portal from "./Portal.svelte";
  import Button from "./Button.svelte";
  import Tag from "../icons/Tag.svelte";
  import FolderPlus from "./FolderPlus.svelte";

  let createProjectData = {
    name: ''
  };
  let createProjectErrors = errors.make(createProjectData);

  let open = false;
  let loading = false;
  async function openModal() {
    open = true;
  }
  async function createProject() {
    loading = true;
    const res = await fetch('/api/projects', {
        method: "POST",
        body: JSON.stringify(createProjectData),
        headers: {
            'Content-Type': "application/json"
        }
    });

    if (!res.ok) {
      createProjectErrors = errors.apply(createProjectErrors, await res.json());
      loading = false;
      return
    }

    const { project } = await res.json();

    push(`#/admin/project/${project.id}`);
    loading = false;
  }
</script>

<button on:click={openModal}>
  <span> Create project + </span>
</button>
{#if open}
  <Portal>
    <div use:clickOutside on:click_outside={() => (open = false)} class="hidden" class:open={open}>
        <form class="modal-content" on:submit={createProject}>
            <span>New project</span>
            <Input type="text" bind:value={createProjectData.name} placeholder="Project name" error={createProjectErrors.name}>
                <Tag/>
            </Input>
            <Button loading={loading} text="Create">
              <FolderPlus />
            </Button>
        </form>
    </div>
  </Portal>
{/if}

<style>
  button {
    background-color: #fff;
    border: none;
    border-radius: 8px;
    padding: 8px;
    cursor: pointer;
  }
  button:hover {
    background-color: var(--hover-bg);
  }
  button > span {
    color: var(--sec-content);
    font-size: 16px;
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
    font-weight: 500;
  }
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
</style>
