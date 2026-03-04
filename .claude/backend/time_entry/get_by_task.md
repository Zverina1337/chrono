# get_time_entries_by_task

Получение всех записей времени задачи.

## Tauri-команда

```
get_time_entries_by_task
```

## Входные данные

| Параметр   | Тип      | Описание    |
|------------|----------|-------------|
| `taskUuid` | `string` | UUID задачи |

## Выходные данные

Массив объектов `TimeEntry`:

```json
[
  {
    "uuid": "...",
    "taskUuid": "...",
    "startedAt": "2026-03-02 10:00:00",
    "endedAt": "2026-03-02 11:30:00",
    "durationMinutes": 90,
    "description": "Работа над задачей",
    "createdAt": "2026-03-02 10:00:00"
  }
]
```

| Поле              | Тип               | Описание                                           |
|-------------------|-------------------|----------------------------------------------------|
| `uuid`            | `string`          | UUID v4, первичный ключ                            |
| `taskUuid`        | `string`          | UUID задачи                                        |
| `startedAt`       | `string`          | UTC datetime начала отсчёта (`YYYY-MM-DD HH:MM:SS`) |
| `endedAt`         | `string \| null`  | UTC datetime конца; `null` = таймер ещё запущен    |
| `durationMinutes` | `number \| null`  | Продолжительность в минутах; `null` = ещё идёт     |
| `description`     | `string`          | Описание записи (по умолчанию `""`)                |
| `createdAt`       | `string`          | UTC datetime создания записи                       |

## Сортировка

По `started_at DESC` (новейшие первые).

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

Если задача не существует — возвращается пустой массив (не ошибка).

## Пример вызова (TypeScript)

```ts
const entries = await invoke<TimeEntry[]>("get_time_entries_by_task", {
  taskUuid: "..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/time_entry_cmd.rs` → `get_time_entries_by_task`
- Репозиторий: `src-tauri/src/repository/time_entry_repo.rs` → `get_by_task()`
