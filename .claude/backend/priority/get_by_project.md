# get_priorities_by_project

Получение всех приоритетов проекта.

## Tauri-команда

```
get_priorities_by_project
```

## Входные данные

| Параметр      | Тип      | Описание     |
|---------------|----------|--------------|
| `projectUuid` | `string` | UUID проекта |

## Выходные данные

Массив объектов `Priority`:

```json
[
  {
    "uuid": "...",
    "projectUuid": "...",
    "name": "Низкий",
    "color": "#6b7280",
    "level": 0,
    "createdAt": "2026-03-02 10:00:00",
    "updatedAt": "2026-03-02 10:00:00"
  }
]
```

| Поле          | Тип      | Описание                                  |
|---------------|----------|-------------------------------------------|
| `uuid`        | `string` | UUID v4, первичный ключ                   |
| `projectUuid` | `string` | UUID проекта-владельца                    |
| `name`        | `string` | Название приоритета                       |
| `color`       | `string` | HEX-цвет                                 |
| `level`       | `number` | Числовой уровень важности (0 = низкий)   |
| `createdAt`   | `string` | UTC datetime создания                     |
| `updatedAt`   | `string` | UTC datetime последнего изменения         |

## Дефолтные приоритеты

При создании проекта автоматически создаются 4 приоритета:

| level | name     | color     |
|-------|----------|-----------|
| 0     | Низкий   | `#6b7280` |
| 1     | Средний  | `#3b82f6` |
| 2     | Высокий  | `#f59e0b` |
| 3     | Срочный  | `#ef4444` |

## Сортировка

По `level ASC`.

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

Если проект не существует — возвращается пустой массив (не ошибка).

## Пример вызова (TypeScript)

```ts
const priorities = await invoke<Priority[]>("get_priorities_by_project", {
  projectUuid: "550e8400-..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/priority_cmd.rs` → `get_priorities_by_project`
- Репозиторий: `src-tauri/src/repository/priority_repo.rs` → `get_by_project()`
