<script>
	import { onMount, tick } from 'svelte';
	import * as wurbo from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../composed-wallet.wasm?url';

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

	onMount(async () => {
		// ensure you are in the Browser environment to rollup your WIT Component
		// const { load } = await import('rollup-plugin-wit-component');

		let listener = new wurbo.Listener();

		console.log({ listener });

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		// define the import handles you are giving to your component
		let all_importables = [
			{ 'seed-keeper:wit-ui/wurbo-in': importables.buildCodeString(listener.namespace) },
			{
				'seed-keeper:wallet/config': importables.buildConfigString({
					username: 'DougAnderson444',
					password: 'password12345678'
				})
			}
		];

		// load the import handles into the Wasm component and get the ES module returned
		mod = await load(wasmBytes, all_importables);

		// call `render` with your inputs for the component
		let data = {
			tag: 'all-content',
			val: {
				page: { title: "Let's generate a key and encrypt it with a username and password." },
				input: { placeholder: 'Enter a Username here' }
			}
		};
		renderedHTML = mod.wurboOut.render(data);

		// lisen for events from the component
		listener.listen(mod);
	});

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (renderedHTML && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			// once the DOM has our elements loaded, we can activate the event emitters
			mod.wurboOut.activate();
		})();
</script>

<svelte:head>
	<title>Seed Keeper</title>
	<!-- The CSS is embedded in the HTML templates which is embedded in the wasm right now -->
	<!-- TODO: instead of embedding the HTML, takes them as props so we can run them through Tailwindcss first -->
	<!-- TODO: instead of embedding the HTML, takes them as props so we can run them through Tailwindcss first -->
	<script src="https://cdn.tailwindcss.com"></script>
</svelte:head>
<div>
	{#if renderedHTML}
		{@html renderedHTML}
	{/if}
</div>
