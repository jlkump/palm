<script lang="ts">
  let { children } = $props();
</script>

<style>
    :global(body) {
		margin: 0;
        background-color: #f2f5f7;
        --paper: #f2f5f7;
        --accent: oklch(35.135% 0.00138 198.686);
        --accent-button: oklch(15.434% 0.00002 271.152);
        --item: #dadcdd;
        --item-border: #9b9d9e;

        --email: rgb(124, 123, 123);

        --disabled: #adadad;
        --warning: rgb(187, 137, 43);
        --error: rgb(192, 25, 42);
        --error-bg: rgba(192, 25, 42, 0.384);
	}

    .page {
        background-color: var(--paper);
        min-height: calc(100vh - 100px);

        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>

<div class="page">
    {@render children()}
</div>