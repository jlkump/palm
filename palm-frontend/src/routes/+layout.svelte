<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
    import Titlebar from '$lib/components/Titlebar.svelte';
    import Page from '$lib/components/Page.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<Titlebar />
<Page>
	{@render children()}
</Page>
