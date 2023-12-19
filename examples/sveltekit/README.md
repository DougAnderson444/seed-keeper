# Wurbo SvelteKit Example

1. Add `wurbo` and `rollup-plugin-wit-component` to your SvelteKit project:

```bash
npm install wurbo rollup-plugin-wit-component -D
```

2. Add a minijinja template:

```html
<div id="{{page.id}}" class="p-4">
	<h1 class="text-red-600 text-2xl font-bold">{{page.title}}</h1>
	<div>{% include "input.html" %}</div>
	<div>{% include "output.html" %}</div>
</div>
```

3. Add [TailwindCSS](https://tailwindcss.com/docs/guides/sveltekit)	as CDN because the CSS is embedded in the Wasm binary templates (TODO: pull in templates for the css can be generated):

```js
<script src="https://cdn.tailwindcss.com"></script>
```

4. export `buildCodeString` from `./importables.js` to satisfy the `wurbo-in` interface:

```js
export function buildCodeString(namespace) {
	return `
      const CHANNEL_NAME = 'listener_updates';
      const bc = new BroadcastChannel(CHANNEL_NAME);
      export function addeventlistener({ selector, ty }) {
        document.querySelector(selector).addEventListener(ty, (e) => {
          let ctx = {
            tag: e.target.name,
            val: {
              value: e.target.value,
            }
          };
          bc.postMessage(window.${namespace}.render(ctx));
        });
      }`;
}
```

5. Wire up the exports of `seed-keeper-wit` to the imports of `seed-keeper-wit-ui` usign [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools):

```bash
cargo install wasm-tools
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
