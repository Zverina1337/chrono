# Chrono — Project Context

## Описание
Десктопное приложение для пассивного тайм-трекинга.

## Стек
- **Runtime:** Tauri v2 (Rust backend)
- **Frontend:** Vue 3 (Composition API, `<script setup>`) + TypeScript
- **Стили:** UnoCSS (presetWind + presetIcons)
- **Линтер:** ESLint (flat config, `eslint.config.ts`)
- **Форматтер:** Prettier

## Команды
```bash
pnpm dev        # запуск в режиме разработки (Tauri + Vite)
pnpm build      # production сборка
pnpm lint       # ESLint с автофиксом
pnpm format     # Prettier форматирование src/**
pnpm typecheck  # TypeScript проверка без компиляции
```

## Структура
```
src/            # Vue frontend
  main.ts       # точка входа, подключает UnoCSS
  App.vue       # корневой компонент
src-tauri/      # Rust backend (Tauri)
  src/lib.rs    # Tauri команды
  tauri.conf.json
uno.config.ts   # конфиг UnoCSS
eslint.config.ts # ESLint flat config
vite.config.ts  # Vite + UnoCSS плагин
```

## Архитектура
- Функциональный стиль, Composition API
- Принципы SOLID и Clean Architecture
- Комментарии в коде на русском языке
- Отступы: 2 пробела, двойные кавычки, точка с запятой

## Текущее состояние
Минимальное ядро: Tauri v2 + Vue 3 + UnoCSS + ESLint + Prettier.
Библиотеки состояния, роутер, ORM — не подключены.
