import { defineConfig } from "unocss";
import { presetWind, presetIcons, presetAttributify } from "unocss";

export default defineConfig({
  content: {
    pipeline: {
      include: [/\.(vue|svelte|[jt]sx|mdx?|astro|html)($|\?)/, "src/**/*.{js,ts}"],
    },
  },
  darkMode: "media",
  preflights: [
    {
      getCSS: () => `
        @font-face {
          font-family: 'Geist';
          src: url('/fonts/Geist-Regular.ttf') format('truetype');
          font-weight: 400;
          font-style: normal;
        }

        * {
          font-family: 'Geist'
        }
      `,
    },
  ],
  theme: {
    colors: {
      dark: {
        base: "#08080e",
        emerald: "#2dd4a0",
        indigo: "#5b6aff",
        violet: "#9d6ef8",
        amber: "#f5a623",
        rose: "#f0506e",
        surface: {
          DEFAULT: "#0d0d11",
          glass: "rgba(10,10,16,.70)",
          pill: "rgba(18,18,26,.90)",
          card: "rgba(255,255,255,.04)",
        },
      },
      white: {
        base: "#f0f0f3",
        emerald: "#16a07a",
        indigo: "#4f5de8",
        violet: "#7c3aed",
        amber: "#d97706",
        rose: "#e03556",
        surface: {
          DEFAULT: "#1c1c22",
          glass: "#ebebee",
          pill: "rgba(18,18,26,.90)",
          card: "#ffffff",
        },
      },

      // text
      primary: "rgba(255,255,255,.92)",
      secondary: "rgba(255,255,255,.82)",
      tertiary: "rgba(255,255,255,.45)",
      muted: "rgba(255,255,255,.28)",
      disabled: "rgba(255,255,255,.18)",
    },
    borderRadius: {
      xs: "4px",
      sm: "6px",
      md: "8px",
      lg: "10px",
      xl: "12px",
      "2xl": "16px",
    },
    boxShadow: {
      low: "0 2px 12px rgba(0,0,0,.30)",
      mild: "0 4px 24px rgba(0,0,0,.50)",
      glow: "0 4px 20px rgba(45, 212, 160, 0.40)",
    },
    fontFamily: {
      Geist: "Geist",
    },
  },
  presets: [presetWind({ preflight: "on-demand" }), presetIcons(), presetAttributify()],
});
