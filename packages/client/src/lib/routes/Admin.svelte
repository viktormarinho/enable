<script lang=ts>
  import Navbar from "../components/Navbar.svelte";
  import NewProject from "../components/NewProject.svelte";
  import Projects from "../components/Projects.svelte";

</script>

<div>
  <Navbar />
  <div class="page">
    <header class="page-header">
      <h1>Projects</h1>
      <NewProject />
    </header>
    <Projects />
  </div>
</div>