import { render } from "solid-js/web";

export function main() {
	console.log("Hello from Typescript!");
	window.wasmBindings.print_from_rust("Wasm loaded. Hello from WASM!");

	function App() {
		return (
			<div class="HelloWorld">
				<h1>Hello, World! </h1>
			</div>
		);
	}

	render(() => <App />, document.getElementById("root") as HTMLElement);
}
