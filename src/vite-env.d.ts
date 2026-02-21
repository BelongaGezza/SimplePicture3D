// Copyright (c) 2026 SimplePicture3D Contributors
// SPDX-License-Identifier: MIT

/// <reference types="vite/client" />

declare module "*.svelte" {
  import type { ComponentType } from "svelte";
  const component: ComponentType;
  export default component;
}
