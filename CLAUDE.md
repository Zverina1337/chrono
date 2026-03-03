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
src/              # Vue frontend
  main.ts         # точка входа, подключает UnoCSS
  App.vue         # корневой компонент
src-tauri/src/    # Rust backend (Tauri)
  lib.rs          # Builder, setup(БД), регистрация 30 команд
  main.rs         # точка входа
  error.rs        # AppError (thiserror + Serialize)
  db/
    mod.rs        # init_db(), PRAGMA WAL + foreign_keys
    migrations.rs # SQL-миграции (user_version)
  models/         # Структуры данных + DTO (serde camelCase)
    project.rs, status.rs, priority.rs, task.rs,
    subtask.rs, label.rs, time_entry.rs
  repository/     # CRUD-функции (чистые fn, принимают &Connection)
    project_repo.rs, status_repo.rs, priority_repo.rs,
    task_repo.rs, subtask_repo.rs, label_repo.rs, time_entry_repo.rs
  commands/       # Tauri-команды (тонкие адаптеры над repository)
    project_cmd.rs, status_cmd.rs, priority_cmd.rs,
    task_cmd.rs, subtask_cmd.rs, label_cmd.rs, time_entry_cmd.rs
uno.config.ts     # конфиг UnoCSS
eslint.config.ts  # ESLint flat config
vite.config.ts    # Vite + UnoCSS плагин
```

## Архитектура
- Функциональный стиль, Composition API
- Принципы SOLID и Clean Architecture
- Комментарии в коде на русском языке
- Отступы: 2 пробела, двойные кавычки, точка с запятой

## Стек Rust-бэкенда
- **БД:** SQLite через rusqlite (bundled) + миграции через user_version
- **UUID:** uuid v1 (v4)
- **Ошибки:** thiserror + Serialize для Tauri
- **Состояние:** Mutex<Connection> в Tauri State

## Сущности БД
Projects → Statuses (канбан-колонки) → Tasks → Subtasks
Projects → Priorities → Tasks
Labels ↔ Tasks (many-to-many через task_labels)
Tasks → TimeEntries

## Текущее состояние
SQLite-бэкенд с 8 таблицами и 30 Tauri-командами реализован.
Фронтенд пока не подключён к бэкенду — использует Pinia.
TODO: фильтрация задач по тегам.
