// place files you want to import through the `$lib` alias in this folder.
import { default as Seed } from './Seed.svelte';
import { default as Edwards } from './Edwards.svelte';
import { default as Calculator } from './Calculator.svelte';
import { default as Aggregate } from './Aggregate.svelte';

export const examples = [
	// Seed,
	// Edwards, Calculator,
	Aggregate
];
