<script>
  import Error from "../icons/Error.svelte";

    export let type = 'text';
    export let placeholder = '';
    export let value = '';
    export let help = '';
    export let error = '';
    export let maxLength = '';

    function typeAction(node) {
        node.type = type;
    }
</script>

<div>
    <div class="container" class:error={error.length}>
        <label class="control">
            <slot></slot>
            <input use:typeAction placeholder={placeholder} bind:value={value} maxlength={maxLength} />
        </label>
        {#if error.length}
            <div class="error-p">
                <Error />
                <span>
                    {error}
                </span>
            </div>
        {/if}
    </div>
    {#if help}
        <p class="help">{help}</p>
    {/if}
</div>

<style>
    .control {
        display: flex;
        gap: 1rem;
        box-sizing: border-box;
        width: 100%;
        border-radius: .5rem;
        padding: 1rem 2rem;
        background-color: whitesmoke;
        cursor: text;
        align-items: center;
    }
    input {
        display: block;
        width: 100%;
        border: none;
        background-color: transparent;
        color: var(--main-content);
    }
    input:focus {
        outline: none;
    }
    .help {
        font-size: 12px;
        color: var(--sec-content);
        text-align: start;
        margin: 0;
        padding: 8px;
        padding-bottom: 0;
    }
    .error label {
        border: 2px solid var(--error);
    }
    .error-p {
        display: flex;
        align-items: center;
        gap: 4px;
        color: var(--error);
        font-size: 12px;
        text-align: start;
        margin: 0;
        padding: 4px;
    }
</style>