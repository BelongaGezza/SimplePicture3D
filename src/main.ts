import "./app.css";
import App from "./App.svelte";

const app = document.getElementById("app");
if (!app) throw new Error("Missing #app element");
const mount = new App({ target: app });
export default mount;
