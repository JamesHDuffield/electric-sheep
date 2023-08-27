<script lang="ts">
	import '../app.postcss';
	import { MetaTags } from 'svelte-meta-tags';
	import { page } from '$app/stores';

	export let data; // Exported so that child components/pages can provide data.

	$: metaTags = {
		title: 'Electric Sheep',
		description: 'Blade Runner inspired interview with a potentially dangerous android.',
		...$page.data.metaTagsChild,
	};
</script>

<MetaTags {...metaTags} />

<slot />
