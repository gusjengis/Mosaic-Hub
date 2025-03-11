import * as Plinth from "plinth-web";
import { main } from "./main.tsx";
import "./hello_world.css";

// Wait for wasm_bindgen functions to load before running app.
Plinth.waitForWasm(() => {
    main();
});
