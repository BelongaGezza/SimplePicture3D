/// <reference types="vite/client" />

declare module "*.svelte" {
  interface ComponentOptions {
    target: HTMLElement;
    anchor?: HTMLElement;
    props?: Record<string, unknown>;
    hydrate?: boolean;
    intro?: boolean;
  }
  class Component {
    constructor(options: ComponentOptions);
  }
  const component: typeof Component;
  export default component;
}
