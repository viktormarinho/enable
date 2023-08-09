<script lang="ts">
  import { onMount } from "svelte";
  import { push } from "svelte-spa-router";
  import Folder from "../icons/Folder.svelte";

  let projects: {
    id: number;
    name: string;
    feature_count: number;
  }[] = [];

  onMount(async () => {
    const res = await fetch("/api/projects");
    const { projects: _projects } = await res.json();
    projects = _projects;
  });
</script>

<div class="projects">
  {#each projects as project}
    <button
      on:click={() => push(`#/admin/project/${project.id}`)}
      class="project"
    >
      <span><Folder /> {project.name}</span>
      <span>{project.feature_count} features</span>
    </button>
  {/each}
</div>

<style>
  .projects {
    margin-top: 32px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .project {
    border: none;
    color: var(--sec-content);
    font-weight: 500;
    background-color: white;
    cursor: pointer;
    width: 50%;
    margin: auto;
    border-radius: 8px;
    display: flex;
    justify-content: space-between;
    padding: 20px 10px;
    align-items: center;
  }
  .project:hover {
    background-color: whitesmoke;
  }
  .project > span {
    color: var(--sec-content);
    font-size: 18px;
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
