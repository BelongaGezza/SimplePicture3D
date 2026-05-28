// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

import "./app.css";
import { mount } from "svelte";
import App from "./App.svelte";

const target = document.getElementById("app");
if (!target) throw new Error("Missing #app element");
mount(App, { target });
