import { render } from "solid-js/web";
import { loadLogs } from "./fetch.tsx";

export function main() {
	loadLogs();

	// function App() {
	// 	return (
	// 		<div class="HelloWorld">
	// 			<h1>Hello, World! </h1>
	// 		</div>
	// 	);
	// }

	// render(() => <App />, document.getElementById("root") as HTMLElement);
}
