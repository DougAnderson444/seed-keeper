<script>
	import { onMount, tick } from 'svelte';
	import * as wurbo from 'wurbo';

	// Import wasm component bytes as a url
	import wasmURL from '../../../aggregate.wasm?url';

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

		// get your wasm bytes from your storage source
		let wasmBytes = await fetch(wasmURL).then((res) => res.arrayBuffer());

		// define the import handles you are giving to your component
		let all_importables = [
			{ 'component:wurbo/wurbo-in': importables.buildCodeString(listener.namespace) },
			{ 'seed-keeper:wit-ui/wurbo-in': importables.buildCodeString(listener.namespace) },
			{ 'example:edwards-ui/wurbo-in': importables.buildCodeString(listener.namespace) },
			{ 'wallet:aggregate-wit-ui/wurbo-in': importables.buildCodeString(listener.namespace) }
		];

		// load the import handles into the Wasm component and get the ES module returned
		mod = await load(wasmBytes, all_importables);

		// call `render` with your inputs for the component
		let data = {
			tag: 'all-content',
			val: {
				app: {
					title: 'Aggregated Wasm User Interfaces'
				},
				seedUi: {
					tag: 'all-content',
					val: {
						page: {
							title: 'UI #1: A Seed Keeper'
						},
						input: {
							placeholder: 'Your Username (pick any 8+ chars)'
							// to load a key, just pass it in as a Uint8Array:
							// encrypted: new Uint8Array([
							// 	80, 253, 160, 118, 34, 159, 76, 149, 169, 213, 236, 57, 204, 35, 47, 123, 207, 233,
							// 	150, 41, 16, 111, 123, 12, 220, 76, 217, 199, 47, 51, 44, 135, 223, 50, 98, 187,
							// 	119, 144, 218, 99
							// ])
						},
						load: null,
						output: null
					}
				},
				edwardsUi: {
					tag: 'all-content',
					val: {
						page: {
							title: 'UI #2: An Edwards25519 Signer'
						},
						input: {
							placeholder: 'a message to sign'
						},
						output: null
					}
				}
			}
		};
		renderedHTML = mod.wurboOut.render(data);

		// lisen for events from the component
		listener.listen(mod);

		// Set up a broadcast channel to listen for updates from the Blob URLs
		const bc = new BroadcastChannel(listener.namespace);

		// Listen for messages from the Blob URLs
		bc.onmessage = (event) => {
			// console.log('Svelte BroadcastChannel evt', { event });
		};
	});

	// Once the HTML is rendered and the module is loaded, we can activate the event emitters
	$: if (renderedHTML && mod)
		(async () => {
			// wait for the DOM to reflect the updates first
			await tick();
			// once the DOM has our elements loaded, we can activate the event emitters
			// mod.wurboOut.activate();
			mod.aggregation.activates();
		})();
</script>

<svelte:head>
	<title>Seed Keeper</title>
	<!-- <script src="https://cdn.tailwindcss.com"></script> -->
</svelte:head>
<div>
	<h1>Aggregate User Interfaces</h1>
	{#if renderedHTML}
		{@html renderedHTML}
	{/if}
</div>
