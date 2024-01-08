export function buildCodeString(namespace) {
	return `
      const bc = new BroadcastChannel('${namespace}');
      export function addeventlistener({ selector, ty }) {
        document.querySelector(selector).addEventListener(ty, (e) => {

          // detect if form event
          if(e.target.closest('form')) {
            e.preventDefault();
          }

          let ctx = {
            tag: e.target.name,
            val: {
              value: e.target.value,
            }
          };

          let el = e.target.closest('[data-slot]');
          if(el) {
            ctx = { tag: el.dataset.slot, val: ctx };
            el = el.closest('[data-slot]');
          }

          let rendered = window.${namespace}.render(ctx); 

          // console.log({ctx}, {rendered});
          bc.postMessage(rendered);
        });
      }`;
}

/**
 * We also need to export a string of JavaScript that is the getConfig() function which returns a Credentials object with the username and password bytes, and an optional `encrypted` Uint8Array
 *
 * @param {Uint8Array} username
 * @param {Uint8Array} password
 * @param {Uint8Array} Optional encrypted seed to load into the wallet. Otherwise if not given (null) the wallet will generate a new seed.
 *
 * @returns {string} A string of JavaScript that is the getConfig() function which returns a Credentials object with the username and password bytes, and an optional `encrypted` Uint8Array
 * */
export function buildConfigString({ username, password, encrypted }) {
	// Convert the given strings to Uint8Arrays
	username = new TextEncoder().encode(username);
	password = new TextEncoder().encode(password);
	encrypted = encrypted ? new Uint8Array(encrypted) : null;

	// Return the string of JavaScript that is the getConfig() function which returns a Credentials object with the username and password bytes, and an optional `encrypted` Uint8Array
	return `
    export function getConfig() {
      return {
        username: new Uint8Array([${username}]),
        password: new Uint8Array([${password}]),
        encrypted: ${encrypted ? `new Uint8Array([${encrypted}])` : null}
      }
    }
  `;
}

// build a Uint8Array seed from a given string
export function buildGetSeedFunc(str) {
	// start off with a 32 bytes long Uint8Array
	let bytes = new Uint8Array(32);

	// fill 32 bytes of random bytes
	window.crypto.getRandomValues(bytes);

	// return a string of JavaScript that is the getSeed() function which returns a Uint8Array seed
	return `export function getSeed() {
      return new Uint8Array([${bytes.toString()}])
  }
  
  export function setConfig() {}
  export getEncrypted() {}
`;
}
