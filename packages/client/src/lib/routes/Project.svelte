<script lang=ts>
    import { onMount } from "svelte";
    import Navbar from "../components/Navbar.svelte";
    import NewFeature from "../components/NewFeature.svelte";
    import { ProjectPageData } from "../types/project";
    import Features from "../components/Features.svelte";
  import BackToProjects from "../components/BackToProjects.svelte";

    export let params = {
        id: ''
    };

    let data: ProjectPageData = {
        project: {
            name: ''
        },
        features: []
    };

    onMount(async () => {
        const res = await fetch(`/api/features/${params.id}`);
        data = await res.json();
    });
</script>

<div>
    <Navbar />
    <div class="page">
        <BackToProjects />
        <header class="page-header">
            <h1>{data.project.name}</h1>
            <NewFeature projectId={params.id} projectName={data.project.name} />
        </header>
        <Features features={data.features}/>
    </div>
</div>