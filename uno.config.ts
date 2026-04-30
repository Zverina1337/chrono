import { defineConfig } from "unocss";
import { presetWind, presetIcons, presetAttributify } from "unocss";

export default defineConfig({
  content: {
    pipeline: {
      include: [
        /\.(vue|svelte|[jt]sx|mdx?|astro|html)($|\?)/,
        "src/**/*.{js,ts}",
      ],
    },
  },
  preflights: [
    {
      getCSS: () => `
      /* ── Geist Sans ─────────────────────────────────────── */
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 100; src: url('/fonts/Geist-Thin.woff2')       format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 200; src: url('/fonts/Geist-UltraLight.woff2') format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 300; src: url('/fonts/Geist-Light.woff2')      format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 400; src: url('/fonts/Geist-Regular.woff2')    format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 500; src: url('/fonts/Geist-Medium.woff2')     format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 600; src: url('/fonts/Geist-SemiBold.woff2')   format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 700; src: url('/fonts/Geist-Bold.woff2')       format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 800; src: url('/fonts/Geist-Black.woff2')      format('woff2'); }
      @font-face { font-family: 'Geist'; font-style: normal; font-display: swap; font-weight: 900; src: url('/fonts/Geist-UltraBlack.woff2') format('woff2'); }

      /* ── Geist Mono ─────────────────────────────────────── */
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 100; src: url('/fonts/GeistMono-Thin.woff2')       format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 200; src: url('/fonts/GeistMono-UltraLight.woff2') format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 300; src: url('/fonts/GeistMono-Light.woff2')      format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 400; src: url('/fonts/GeistMono-Regular.woff2')    format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 500; src: url('/fonts/GeistMono-Medium.woff2')     format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 600; src: url('/fonts/GeistMono-SemiBold.woff2')   format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 700; src: url('/fonts/GeistMono-Bold.woff2')       format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 800; src: url('/fonts/GeistMono-Black.woff2')      format('woff2'); }
      @font-face { font-family: 'Geist Mono'; font-style: normal; font-display: swap; font-weight: 900; src: url('/fonts/GeistMono-UltraBlack.woff2') format('woff2'); }
      
    
      `,
    },
  ],
  theme: {
    colors: {
      brand: {
        50: "#ecfdf5",
        100: "#d1fae5",
        200: "#a7f3d0",
        300: "#6ee7b7",
        400: "#34d399",
        500: "#10b981",
        600: "#059669",
        700: "#047857",
        800: "#065f46",
        900: "#064e3b",
      },
      surface: {
        nav: "#131313", // тёмная вертикальная навигация (самая левая колонка)
        sidebar: "#f4f4f5", // светлая вторая колонка с фильтрами/проектами
        canvas: "#fafafa", // основной фон контентной области
        card: "#ffffff", // панели задач, поповеры, таймер
        hover: "#f4f4f5", // hover для строк и пунктов меню
        active: "#ecfdf5", // подсветка активной задачи (тонкий зелёный вош)
        overlay: "rgba(15, 23, 42, 0.45)",
      },
      ink: {
        primary: "#18181b", // заголовки, активные пункты
        secondary: "#52525b", // основной текст задач
        tertiary: "#71717a", // подписи, метки секций
        muted: "#a1a1aa", // disabled-стрелки приоритетов, плейсхолдеры
        disabled: "#d4d4d8",
        inverse: "#fafafa", // текст на тёмной навигации
      },
      border: {
        DEFAULT: "#e4e4e7",
        subtle: "#f1f1f3", // разделители строк задач
        strong: "#d4d4d8", // чекбоксы в idle
      },
      // Цвета меток. Каждая — пара fg/bg, оптимизированная под пилюли.
      tag: {
        frontend: { fg: "#047857", bg: "#d1fae5" },
        backend: { fg: "#1d4ed8", bg: "#dbeafe" },
        design: { fg: "#6d28d9", bg: "#ede9fe" },
        docs: { fg: "#c2410c", bg: "#ffedd5" },
        bug: { fg: "#b91c1c", bg: "#fee2e2" },
      },
    },
    fontFamily: {
      sans: ["Geist", "system-ui", "-apple-system", "sans-serif"],
      mono: [
        '"Geist Mono"',
        "ui-monospace",
        "SFMono-Regular",
        "monospace",
      ],
      numeric: ['"Geist Mono"', "ui-monospace", "monospace"],
    },
    fontSize: {
      xs: ["0.6875rem", { lineHeight: "1rem" }], // 11px, БЕЗ tracking
      sm: ["0.8125rem", { lineHeight: "1.125rem" }], // 13px — дефолт
      base: ["0.875rem", { lineHeight: "1.25rem" }], // 14px
      md: ["0.9375rem", { lineHeight: "1.375rem" }], // 15px
      lg: ["1.0625rem", { lineHeight: "1.5rem" }], // 17px
      xl: ["1.25rem", { lineHeight: "1.75rem" }], // 20px
      "2xl": ["1.5rem", { lineHeight: "2rem" }],
      clock: [
        "1.375rem",
        { lineHeight: "1.5rem", letterSpacing: "-0.01em" },
      ], // 22px — таймер
    },
    letterSpacing: {
      tight: "-0.01em",
      normal: "0",
      wide: "0.02em",
      wider: "0.04em",
      widest: "0.08em",
    },
    borderRadius: {
      none: "0",
      xs: "4px",
      sm: "6px",
      md: "8px",
      lg: "10px",
      xl: "12px",
      "2xl": "16px",
      pill: "9999px",
      full: "9999px",
    },
    boxShadow: {
      xs: "0 1px 2px 0 rgba(16, 24, 40, 0.04)",
      sm: "0 1px 3px 0 rgba(16, 24, 40, 0.06), 0 1px 2px -1px rgba(16, 24, 40, 0.04)",
      md: "0 4px 8px -2px rgba(16, 24, 40, 0.06), 0 2px 4px -2px rgba(16, 24, 40, 0.04)",
      lg: "0 10px 24px -4px rgba(16, 24, 40, 0.08), 0 4px 8px -4px rgba(16, 24, 40, 0.04)",
      card: "0 1px 2px rgba(16, 24, 40, 0.04), 0 1px 3px rgba(16, 24, 40, 0.06)",
      popover:
        "0 12px 32px -8px rgba(16, 24, 40, 0.18), 0 4px 12px -4px rgba(16, 24, 40, 0.08)",
      timer:
        "0 8px 24px -6px rgba(16, 24, 40, 0.20), 0 2px 6px -2px rgba(16, 24, 40, 0.08)",
      focus: "0 0 0 3px rgba(52, 211, 153, 0.35)",
    },
    transitionDuration: {
      fast: "150ms",
    },
  },
  shortcuts: [
    // ── Иерархия ──────────────────────────────────────────────────
    [
      "text-h1",
      "text-xl font-semibold tracking-tight text-ink-primary",
    ],
    ["text-h2", "text-md font-semibold text-ink-primary"],

    // ── Тело UI (Geist Sans наследуется от body) ─────────────────
    ["text-body", "text-md text-ink-primary"],
    ["text-body-strong", "text-md font-medium text-ink-primary"],
    ["text-body-muted", "text-md text-ink-tertiary"],

    // ── Подписи и метки ──────────────────────────────────────────
    ["text-caption", "text-xs text-ink-tertiary"],
    [
      "text-label",
      "text-sm font-medium uppercase tracking-wider text-ink-muted",
    ],
    ["text-rail", "text-xs uppercase tracking-widest text-ink-muted"],

    // ── Время задачи ─────────────────────────────────────────────
    [
      "text-time",
      "text-sm font-medium tabular-nums text-ink-primary",
    ],
    ["text-time-muted", "text-sm tabular-nums text-ink-muted"],

    // ── Часы таймера (Geist Mono) ────────────────────────────────
    [
      "text-clock",
      "font-mono text-clock font-semibold tabular-nums tracking-tight text-ink-primary",
    ],

    // ── Счётчики и цифры в badge ─────────────────────────────────
    [
      "text-counter",
      "text-xs font-medium tabular-nums text-ink-muted",
    ],

    // ── Метки (теги) ────────────────────────────────────────────
    [
      "tag",
      "inline-flex items-center px-2 h-tag-h rounded-md text-xs font-medium leading-none whitespace-nowrap",
    ],
    ["tag-frontend", "tag bg-tag-frontend-bg text-tag-frontend-fg"],
    ["tag-backend", "tag bg-tag-backend-bg  text-tag-backend-fg"],
    ["tag-design", "tag bg-tag-design-bg   text-tag-design-fg"],
    ["tag-docs", "tag bg-tag-docs-bg     text-tag-docs-fg"],
    ["tag-bug", "tag bg-tag-bug-bg      text-tag-bug-fg"],

    // ── Контролы ─────────────────────────────────────────────────
    ["text-tag", "text-xs font-medium leading-none"],
    ["text-button", "text-sm font-medium"],
    [
      "text-input",
      "text-sm text-ink-primary placeholder:text-ink-muted",
    ],
    ["text-kbd", "text-xs font-medium text-ink-muted"],

    // ── Зачёркнутые готовые задачи ───────────────────────────────
    [
      "text-strike",
      "text-sm line-through decoration-ink-muted text-ink-muted",
    ],
    ["nav-w", "w-[64px]"],
    ["sidebar-w", "w-[240px]"],
    ["header-h", "h-[64px]"],
    ["task-row-h", "h-[44px]"],
    ["tag-h", "h-[20px]"],
    ["btn-sm-h", "h-[28px]"],
    ["btn-md-h", "h-[36px]"],
    ["btn-lg-h", "h-[40px]"],
    ["timer-h", "h-[48px]"],
    ["timer-w", "w-[320px]"],

    // ── Кнопки ──────────────────────────────────────────────────
    [
      "btn",
      "inline-flex items-center justify-center gap-1.5 font-medium rounded-md transition-colors duration-fast disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:shadow-focus",
    ],
    ["btn-sm", "btn h-btn-sm-h px-2.5 text-sm"],
    ["btn-md", "btn h-btn-md-h px-3.5 text-sm"],
    ["btn-lg", "btn h-btn-lg-h px-4 text-md"],
    [
      "btn-primary",
      "bg-brand-500 text-white hover:bg-brand-600 active:bg-brand-700 shadow-xs",
    ],
    [
      "btn-secondary",
      "bg-surface-card text-ink-primary border border-border hover:bg-surface-hover",
    ],
    [
      "btn-ghost",
      "text-ink-secondary hover:text-ink-primary hover:bg-surface-hover",
    ],
    [
      "btn-icon",
      "btn w-btn-md-h h-btn-md-h p-0 btn-ghost rounded-md",
    ],
    // ── Заголовок секции (uppercase подписи: ФИЛЬТРЫ, ПРОЕКТЫ) ─
    [
      "section-label",
      "text-xs font-semibold uppercase tracking-wider text-ink-tertiary",
    ],

    [
      "card",
      "bg-surface-card rounded-xl border border-border shadow-card",
    ],
    ["panel", "bg-surface-card rounded-lg border border-border"],

    // ── Поля ввода ─────────────────────────────────────────────
    [
      "input-base",
      "h-btn-md-h px-3 text-sm bg-surface-card border border-border rounded-md text-ink-primary placeholder:text-ink-muted transition-colors duration-fast focus:outline-none focus:border-brand-400 focus:shadow-focus",
    ],
    ["input-search", "input-base pl-9 pr-12"],

    // ── Чекбокс (контейнер) ────────────────────────────────────
    [
      "checkbox",
      "inline-flex items-center justify-center w-[18px] h-[18px] rounded border border-border-strong hover:border-brand-500 transition-colors duration-fast cursor-pointer flex-shrink-0",
    ],
    [
      "checkbox-checked",
      "checkbox bg-brand-500 border-brand-500 text-white",
    ],

    // ── Сегментный переключатель (Список / Канбан) ─────────────
    [
      "segment",
      "inline-flex items-center p-0.5 bg-surface-sidebar rounded-md",
    ],
    [
      "segment-item",
      "inline-flex items-center justify-center h-7 px-3 text-sm font-medium rounded text-ink-secondary cursor-pointer transition-colors duration-fast",
    ],
    [
      "segment-item-active",
      "bg-surface-card text-ink-primary shadow-xs",
    ],

    // ── Строка задачи ──────────────────────────────────────────
    [
      "task-row",
      "group flex items-center gap-3 h-task-row-h px-4 border-b border-border-subtle hover:bg-surface-hover transition-colors duration-fast",
    ],
    ["task-row-active", "bg-surface-active"],
    ["task-row-done", "opacity-60 line-through decoration-ink-muted"],

    // ── Пункты сайдбара ────────────────────────────────────────
    [
      "nav-item",
      "flex items-center gap-2 h-9 px-3 rounded-md text-sm text-ink-secondary hover:bg-surface-card hover:text-ink-primary cursor-pointer transition-colors duration-fast",
    ],
    [
      "nav-item-active",
      "bg-surface-card text-ink-primary font-medium",
    ],

    // ── Пункт тёмной навигации (узкая колонка слева) ───────────
    [
      "rail-item",
      "flex items-center justify-center w-10 h-10 rounded-md text-ink-muted hover:text-ink-inverse hover:bg-white/5 cursor-pointer transition-colors duration-fast",
    ],
    ["rail-item-active", "text-ink-inverse bg-white/10"],

    // ── Виджет таймера (плавающая капсула снизу справа) ────────
    [
      "timer-widget",
      "fixed bottom-4 right-4 inline-flex items-center gap-3 h-timer-h px-4 bg-surface-card rounded-pill shadow-timer border border-border",
    ],
    [
      "timer-digits",
      "font-numeric text-timer font-semibold tabular-nums tracking-tight text-ink-primary",
    ],

    // ── Хинт горячей клавиши ───────────────────────────────────
    [
      "kbd",
      "inline-flex items-center justify-center min-w-5 h-5 px-1.5 text-xs font-medium text-ink-tertiary bg-surface-sidebar border border-border rounded",
    ],
  ],
  rules: [
    // Произвольная толщина границы дробным значением (border-1.5).
    [
      /^border-(\d+(?:\.\d+)?)$/,
      ([, w]) => ({ "border-width": `${w}px` }),
    ],

    // Tabular-nums как утилита, чтобы цифры таймера не «прыгали».
    ["tabular-nums", { "font-variant-numeric": "tabular-nums" }],
  ],

  // Гарантированно генерируем динамические классы (имена тегов
  // приходят из БД, поэтому статический сканер их не поймает).
  safelist: [
    ...["frontend", "backend", "design", "docs", "bug"].map(
      (t) => `tag-${t}`,
    ),
    ...["working", "backlog", "done"].map((s) => `status-dot-${s}`),
    ...["high", "medium", "low"].map((p) => `text-priority-${p}`),
    "i-mdi-check",
    "i-mdi-arrow-up",
    "i-mdi-arrow-right",
    "i-mdi-arrow-down",
    "i-mdi-magnify",
    "i-mdi-plus",
    "i-mdi-cog-outline",
    "i-mdi-clock-outline",
    "i-mdi-folder-outline",
    "i-mdi-chart-bar",
    "i-mdi-view-list",
    "i-mdi-view-column",
  ],

  presets: [presetWind({ preflight: "on-demand" }), presetIcons()],
});
