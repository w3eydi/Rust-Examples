import App from './App.svelte';
import wasm from "../../rust/Cargo.toml";

const init = async () => {
	const bindings = await wasm();
	const app = new App({
	target: document.body,
	props: {
			  bindings,
			},
  });
};

init();