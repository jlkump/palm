<script lang="ts">
  let { children } = $props();
</script>

<style>
    :global(body) {
		margin: 0;
        background-color: #f2f5f7;
        --accent: oklch(35.135% 0.00138 198.686);
        --accent-button: oklch(15.434% 0.00002 271.152);
	}

    .page {
        background-color: #f2f5f7;
        min-height: calc(100vh - 100px);

        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>

<div class="page">
    {@render children()}
</div>