<script lang=ts>
    import { onMount } from "svelte";
    import Navbar from "../components/Navbar.svelte";

    export let params = {
        id: ''
    };

    let data = {
        project: '',
        features: []
    };

    onMount(async () => {
        const res = await fetch(`/api/features?project_id`);
        const { features } = await res.json();
    });
</script>

<div>
    <Navbar />
    <div class="page">
        <header class="page-header">
            <h1>{project.name}</h1>
        </header>
    </div>
</div>