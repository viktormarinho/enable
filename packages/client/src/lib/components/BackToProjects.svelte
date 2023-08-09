<script lang=ts>
    import Arrow from "../icons/Arrow.svelte";
</script>

<a href="#/admin" class="back hover">
  <Arrow />
  <span>Back to projects</span>
</a>

<style>
    .back {
        display: flex;
        align-items: center;
        width: fit-content;
        color: var(--sec-content);
        gap: 4px;
        border-radius: 8px;
        padding: 4px;
        text-decoration: none;
    }
</style>