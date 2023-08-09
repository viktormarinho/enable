<script lang=ts>
    import ApiIcon from "../icons/Api.svelte";

    export let selected = false;
</script>

<button class:selected={selected} on:click class="btn-secondary">
    <span>
        <ApiIcon />
    </span>
</button>

<style>
    button {
        height: 2.5rem;
        width: 2.5rem;
        display: flex;
        align-items: center;
        margin-left: auto;
        justify-content: center;
    }
    span {
        padding-top: 2px;
    }
    .selected {
        border: 2px solid dodgerblue;
        background-color: rgb(195, 222, 250);
    }
    .selected:hover {
        background-color: rgb(145, 195, 250);
    }
    .selected > span {
        color: dodgerblue;
    }
</style>