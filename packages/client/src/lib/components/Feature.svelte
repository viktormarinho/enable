<script lang=ts>
  import { EnvironmentFeature } from "../types/feature";
  import ApiButton from "./ApiButton.svelte";
  import ApiDefinitions from "./ApiDefinitions.svelte";
  import CopyButton from "./CopyButton.svelte";
  import DeleteButton from "./DeleteButton.svelte";

  export let feature: EnvironmentFeature;

  let showApiDefs = false;
</script>

<div class="feature-container">
  <div class="feature">
        <CopyButton text={feature.feature_id} />
        <span class="name">{feature.feature_id}</span>
        <ApiButton on:click={() => (showApiDefs = !showApiDefs)} selected={showApiDefs}/>
        <DeleteButton label={`${feature.feature_id} from all environments`} on:confirm />
        <span class="tag" class:tag-active={feature.active}
          >{feature.active ? "Active" : "Disabled"}</span
        >
        <button
          class="toggle"
          class:active={feature.active}
          on:click
        >
          <div class="ball" />
        </button>
      </div>
      <div class:show={showApiDefs} class="no-h">
        <ApiDefinitions name={feature.feature_id} active={feature.active}/>
      </div>
</div>

<style>
  .feature-container {
    border-bottom: 2px solid var(--sec-content);
  }
  .feature-container:last-child {
    border-bottom: none;
  }
  .feature {
    padding: 16px;
    display: flex;
    gap: 16px;
    justify-content: space-between;
    align-items: center;
  }
  .toggle {
    border-radius: 16px;
    cursor: pointer;
    border: 2px solid var(--main-content);
    padding: 2px;
    padding-right: 20px;
    transition: 200ms ease all;
    background-color: white;
  }
  .toggle:hover {
    transform: scale(1.05);
  }
  .ball {
    width: 1rem;
    height: 1rem;
    border-radius: 9999px;
    /* border: 2px solid var(--main-content); */
    background-color: var(--main-content);
  }
  .active {
    padding-left: 20px;
    padding-right: 2px;
  }
  .tag {
    border-radius: 8px;
    padding: 8px;
    text-transform: uppercase;
    background-color: rgb(247, 189, 189);
    color: red;
    font-size: 12px;
    width: 70px;
    text-align: center;
  }
  .tag-active {
    background-color: rgb(189, 220, 252);
    color: dodgerblue;
  }
  .name {
    font-family: monospace;
    font-weight: bold;
    padding: 8px;
    border-radius: 8px;
    background-color: var(--main-content);
    color: white;
  }
  .no-h {
    height: 1px;
    overflow: hidden;
    transition: 300ms ease all;
  }
  .show {
    height: 275px;
  }
</style>