# get_all_projects

Получение списка всех проектов.

## Tauri-команда

```
get_all_projects
```

## Входные данные

Нет параметров.

## Выходные данные

Массив объектов `Project`:

```json
[
  {
    "uuid": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Мой проект",
    "description": "Описание проекта",
    "position": 0,
    "createdAt": "2026-03-02 10:00:00",
    "updatedAt": "2026-03-02 10:00:00"
  }
]
```

| Поле          | Тип      | Описание                          |
|---------------|----------|-----------------------------------|
| `uuid`        | `string` | UUID v4, первичный ключ           |
| `name`        | `string` | Название проекта                  |
| `description` | `string` | Описание (по умолчанию `""`)      |
| `position`    | `number` | Порядковый номер для сортировки   |
| `createdAt`   | `string` | UTC datetime создания             |
| `updatedAt`   | `string` | UTC datetime последнего изменения |

## Сортировка

По `position ASC`, затем `created_at ASC`.

## Ошибки

| Ошибка              | Когда                    |
|---------------------|--------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
const projects = await invoke<Project[]>("get_all_projects");
```

## Исходный код

- Команда: `src-tauri/src/commands/project_cmd.rs` → `get_all_projects`
- Репозиторий: `src-tauri/src/repository/project_repo.rs` → `get_all()`
