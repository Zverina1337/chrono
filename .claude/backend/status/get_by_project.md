# get_statuses_by_project

Получение всех статусов (канбан-колонок) проекта.

## Tauri-команда

```
get_statuses_by_project
```

## Входные данные

| Параметр      | Тип      | Описание       |
|---------------|----------|----------------|
| `projectUuid` | `string` | UUID проекта   |

## Выходные данные

Массив объектов `Status`:

```json
[
  {
    "uuid": "...",
    "projectUuid": "...",
    "name": "К работе",
    "color": "#6366f1",
    "position": 0,
    "createdAt": "2026-03-02 10:00:00",
    "updatedAt": "2026-03-02 10:00:00"
  }
]
```

| Поле          | Тип      | Описание                              |
|---------------|----------|---------------------------------------|
| `uuid`        | `string` | UUID v4, первичный ключ               |
| `projectUuid` | `string` | UUID проекта-владельца                |
| `name`        | `string` | Название статуса                      |
| `color`       | `string` | HEX-цвет (по умолчанию `#6366f1`)    |
| `position`    | `number` | Порядок на канбан-доске               |
| `createdAt`   | `string` | UTC datetime создания                 |
| `updatedAt`   | `string` | UTC datetime последнего изменения     |

## Сортировка

По `position ASC`.

## Ошибки

| Ошибка              | Когда                    |
|---------------------|--------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

Если проект не существует — возвращается пустой массив (не ошибка).

## Пример вызова (TypeScript)

```ts
const statuses = await invoke<Status[]>("get_statuses_by_project", {
  projectUuid: "550e8400-..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/status_cmd.rs` → `get_statuses_by_project`
- Репозиторий: `src-tauri/src/repository/status_repo.rs` → `get_by_project()`
