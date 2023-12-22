<script>
	import { onMount, tick } from 'svelte';
	import * as wurbo from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../composed-calculator.wasm?url';

	// get imports from +page.svelte
	export let importables;
	export let load;

	/**
	 * The rendered component as a string of HTML
	 * @type {string | null}
	 */
	let renderedHTML;
	/**
	 * The module that loads the WebAssembly component
	 *
	 * @type {{ render: (arg0: string) => string | null; listen: () => void; }}
	 */
	let mod;

	let sum = 'Calculating...';

	onMount(async () => {
		// ensure you are in the Browser environment to rollup your WIT Component
		// const { load } = await import('rollup-plugin-wit-component');

		let listener = new wurbo.Listener();

		console.log({ listener });

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		// define the import handles you are giving to your component
		let all_importables = [];

		// load the import handles into the Wasm component and get the ES module returned
		mod = await load(wasmBytes, all_importables);

		console.log({ mod });

		sum = mod.calculate.evaluate('2+2');
	});
</script>

<svelte:head>
	<title>Composed Calculator</title>
	<script src="https://cdn.tailwindcss.com"></script>
</svelte:head>
<div>
	<h1 class="text-2xl font-semibold">Composed Calculator</h1>
	<div>
		Sum: {sum}
	</div>
</div>
