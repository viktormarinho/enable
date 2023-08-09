<script lang="ts">
  export let name = "";
  export let active = false;

  let selectedTab: "rest" | "ts-sdk" = "rest";
</script>

<div class="defs">
  <div class="tab-controls">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <span role="tab" tabindex="0" aria-selected={selectedTab === "rest"} on:click={() => (selectedTab = 'rest')}>REST API</span>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <span role="tab" tabindex="0" aria-selected={selectedTab === "ts-sdk"} on:click={() => (selectedTab = 'ts-sdk')}>Typescript SDK</span>
  </div>
  {#if selectedTab === "rest"}
  <div class="api-type-container">
    <div class="container">
      <code>curl -i -H "Authorization: Bearer ENABLE_TOKEN" -X GET {location.origin}/api/feature/{name}</code>
    </div>
    <span class="label">Response</span>
    <code>{JSON.stringify({ active }, null, 2)}</code>
  </div>
  {:else if selectedTab === "ts-sdk"}
  <div class="api-type-container">
    <div class="code">
      <span>import {"{ EnableClient }"} from "@enable/ts-sdk";</span>
      <span
        >export const features = new EnableClient({`{ url: "${location.origin}", token: env.ENABLE_TOKEN }`});</span
      >
      <span />
      <span>...</span>
      <span />
      <span>import {"{ features }"} from "config";</span>
      <span>const isActive = features.isActive("{name}"); // {active}</span>
    </div>
  </div>
  {/if}
</div>

<style>
  .defs {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 20px;
    padding-left: 32px;
    padding-right: 32px;
    background-color: whitesmoke;
    border-radius: 8px;
  }

  .api-type-container {
    padding-top: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .tab-controls {
    display: flex;
    align-items: center;
    overflow: hidden;
    width: 100%;
    border-bottom: 1px solid var(--sec-content);
  }

  .tab-controls > span {
    padding: 8px 16px;
    border-top-left-radius: 8px;
    border-top-right-radius: 8px;
    color: var(--main-content);
    cursor: pointer;
  }
  .tab-controls > span:hover {
    background-color: rgb(230, 230, 230);
    color: var(--sec-content);
  }
  .tab-controls > span[aria-selected="true"] {
    border-bottom: 2.5px solid var(--sec-content);
  }

  .label {
    font-size: 14px;
    color: var(--sec-content);
    font-weight: medium;
  }

  code {
    font-family: monospace;
    font-weight: medium;
    padding: 8px;
    border-radius: 8px;
    background-color: var(--main-content);
    color: white;
  }
  .code {
    font-family: monospace;
    font-weight: medium;
    padding: 8px;
    border-radius: 8px;
    background-color: var(--main-content);
    color: white;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .tag {
    border-radius: 8px;
    padding: 8px;
    text-transform: uppercase;
    background-color: rgb(189, 220, 252);
    color: dodgerblue;
    font-size: 12px;
  }
  .container {
    display: flex;
    align-items: center;
    gap: 16px;
  }
</style>
