<script lang="ts">
  import { Feature } from "../types/feature";
  import FeatureComponent from './Feature.svelte';

  export let features: (Feature & { api_defs?: boolean })[] = [];

  async function toggleFeature(id: string) {
    const res = await fetch("/api/features/toggle", {
      method: "POST",
      body: JSON.stringify({ id }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!res.ok) return;

    features = features.map((f) => {
      if (f.id == id) {
        f.active = !f.active;
      }
      return f;
    });
  }
</script>

<div class="feature-list">
  {#each features as feature}
    <FeatureComponent feature={feature} on:click={() => toggleFeature(feature.id)} />
  {/each}
</div>

<style>
  .feature-list {
    display: flex;
    flex-direction: column;
    padding: 40px;
    gap: 8px;
  }
</style>
