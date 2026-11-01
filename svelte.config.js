import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
  preprocess: vitePreprocess(),
  // Components still use Svelte 4 `export let` / events API until a dedicated migration.
  compilerOptions: {
    compatibility: {
      componentApi: 4,
    },
  },
};
