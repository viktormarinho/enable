<script lang="ts">
  import Credentials from "./Credentials.svelte";

</script>

<div>
  <section>
    <h2>Credentials</h2>
    <p>
      Generate and delete credentials for external access, such as integrating
      the enable API with your applications
    </p>
    <Credentials />
  </section>
</div>

<style>
  section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  p {
    color: var(--sec-content);
    padding: 0;
    margin: 0;
  }
</style>
