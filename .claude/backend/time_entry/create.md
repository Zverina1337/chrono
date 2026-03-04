# create_time_entry

Создание записи времени. Запись с `endedAt = null` означает запущенный таймер.

## Tauri-команда

```
create_time_entry
```

## Входные данные

| Параметр          | Тип               | Обязательно | Описание                                    |
|-------------------|-------------------|:-----------:|---------------------------------------------|
| `taskUuid`        | `string`          | да          | UUID задачи                                 |
| `startedAt`       | `string`          | да          | Время начала `YYYY-MM-DD HH:MM:SS` (UTC)   |
| `endedAt`         | `string \| null`  | нет         | Время конца; `null` = таймер запущен        |
| `durationMinutes` | `number \| null`  | нет         | Продолжительность в минутах                 |
| `description`     | `string`          | да          | Описание (передавай `""` если нет)          |

## Выходные данные

Объект `TimeEntry` (см. `get_by_task.md`).

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Примеры вызова (TypeScript)

```ts
// Запуск таймера
const entry = await invoke<TimeEntry>("create_time_entry", {
  taskUuid: "...",
  startedAt: "2026-03-05 10:00:00",
  endedAt: null,
  durationMinutes: null,
  description: ""
});

// Запись завершённого интервала
const entry = await invoke<TimeEntry>("create_time_entry", {
  taskUuid: "...",
  startedAt: "2026-03-05 10:00:00",
  endedAt: "2026-03-05 11:30:00",
  durationMinutes: 90,
  description: "Реализация фичи"
});
```

## Исходный код

- Команда: `src-tauri/src/commands/time_entry_cmd.rs` → `create_time_entry`
- Репозиторий: `src-tauri/src/repository/time_entry_repo.rs` → `create()`
