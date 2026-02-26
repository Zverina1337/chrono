import { defineConfig } from "unocss";
import { presetWind, presetIcons, presetAttributify } from "unocss";

export default defineConfig({
  presets: [presetWind({ preflight: "on-demand" }), presetIcons(), presetAttributify()],
});
