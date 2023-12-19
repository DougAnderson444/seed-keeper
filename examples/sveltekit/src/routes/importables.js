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
