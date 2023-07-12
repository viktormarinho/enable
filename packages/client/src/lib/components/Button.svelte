<script lang=ts>
    export let text: string;
</script>

<button on:click>
    <slot></slot>
    <span>{text}</span>
</button>

<style>
    button {
        border-radius: .5rem;
        cursor: pointer;
        padding: 1rem 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        color: white;
        background-color: var(--main-content);
        border: none;
        font-size: 16px;
        font-weight: bold;
    }

    button:hover {
        opacity: .9;
    }
</style>