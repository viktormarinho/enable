<script lang="ts">
  import { EnvironmentFeature } from "../types/feature";
  import FeatureComponent from './Feature.svelte';

  export let features: (EnvironmentFeature & { api_defs?: boolean })[] = [];

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

  async function deleteFeature(feat: EnvironmentFeature) {
    const res = await fetch(`/api/features`, {
      method: "DELETE",
      body: JSON.stringify(feat),
      headers: {
        "Content-Type": "application/json"
      }
    });

    if (!res.ok) return;

    features = features.filter(f => f.id !== feat.id);
  }

</script>

<div class="feature-list">
  {#each features as feature}
    <FeatureComponent feature={feature} on:click={() => toggleFeature(feature.id)} on:confirm={() => deleteFeature(feature)}/>
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
