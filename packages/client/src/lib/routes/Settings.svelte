<script lang=ts>
    import BackToProjects from "../components/BackToProjects.svelte";
    import Navbar from "../components/Navbar.svelte";
  import Settings from "../components/Settings.svelte";

</script>
<div>
    <Navbar />
    <div class="page">
        <BackToProjects />
        <header class="page-header">
            <h1>Settings</h1>
        </header>
        <Settings />
    </div>
</div>