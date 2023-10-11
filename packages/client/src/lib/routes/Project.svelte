<script lang=ts>
    import { onMount } from "svelte";
    import Navbar from "../components/Navbar.svelte";
    import NewFeature from "../components/NewFeature.svelte";
    import { ProjectPageData } from "../types/project";
    import Features from "../components/Features.svelte";
    import BackToProjects from "../components/BackToProjects.svelte";
    import Adjustments from "../icons/Adjustments.svelte";
    import { Environment } from "../types/environment";
    import { EnvironmentFeature } from "../types/feature";
    import Plus from "../icons/Plus.svelte";
    import NewEnvironment from "../components/NewEnvironment.svelte";

    export let params = {
        id: ''
    };

    let meta: ProjectPageData = {
        project: {
            name: ''
        },
        envs: []
    };

    let features: EnvironmentFeature[] = [];
    let selectedEnv: Environment | null = null;

    onMount(async () => {
        const metaRes = await fetch(`/api/projects/meta/${params.id}`);
        meta = await metaRes.json();
        selectedEnv = meta.envs[0];
        const envId = selectedEnv?.id; // Check for no environments or maybe do not let user delete last env
        const res = await fetch(`/api/features/${envId}`);
        features = (await res.json()).features;
    });

    async function loadCurrentEnvFeatures(e: any) {
        selectedEnv = meta.envs.find(env => env.id === e.target.value);
        if (!selectedEnv) return;

        const res = await fetch(`/api/features/${selectedEnv.id}`);
        features = (await res.json()).features;
    }
</script>

<div>
    <Navbar />
    <div class="page">
        <BackToProjects />
        <header class="page-header">
            <h1>{meta.project.name}</h1>
            <div class="project-actions">
                <div class="tooltip">
                    <span>
                        Change environment
                    </span>
                    <select on:change={loadCurrentEnvFeatures}>
                        {#each meta.envs as env}
                            <option value={env.id}>{env.name}</option>
                        {/each}
                    </select>
                </div>
                <div class="tooltip">
                    <span>Project settings</span>
                    <button class="btn-secondary">
                        <Adjustments />
                    </button>
                </div>
                <div class="new-container">
                    <button class="btn-secondary">
                        <Plus />
                    </button>
                    <div class="new-options">
                        <NewFeature projectId={params.id} projectName={meta.project.name} />
                        <NewEnvironment projectId={params.id} projectName={meta.project.name} />
                    </div>
                </div>
            </div>
        </header>
        <Features features={features}/>
    </div>
</div>

<style>
    .project-actions {
        display: flex;
        gap: .5rem;
        height: 20px;
        align-items: center;
    }

    .tooltip {
        position: relative;
    }

    .tooltip > span {
        font-size: 12px;
        max-width: 200%;
        width: fit-content;
        text-align: center;
        padding: 8px;
        border-radius: 8px;
        border: 1px solid var(--sec-content);
        position: absolute;
        z-index: 1;
        bottom: 120%;
        left: 0%;
        opacity: 0%;
        visibility: hidden;
        transition-duration: 200ms;
        color: white;
        background-color: var(--main-content);
    }

    .tooltip:hover > span {
        visibility: visible;
        opacity: 100%;
    }

    .new-container {
        position: relative;
    }

    .new-container > .new-options {
        position: absolute;
        top: 120%;
        right: 0%;
        padding: 8px;
        border-radius: 8px;
        visibility: hidden;
        opacity: 0%;
        transition-duration: 200ms;
        display: flex;
        flex-direction: column;
        gap: 8px;
        box-shadow: var(--shadow);
        border: 1px solid #e6e6e6;
        background-color: white;
    }

    .new-container:hover > .new-options {
        visibility: visible;
        opacity: 100%;
    }
</style>