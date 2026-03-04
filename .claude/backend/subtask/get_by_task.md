# get_subtasks_by_task

Получение всех подзадач задачи.

## Tauri-команда

```
get_subtasks_by_task
```

## Входные данные

| Параметр   | Тип      | Описание    |
|------------|----------|-------------|
| `taskUuid` | `string` | UUID задачи |

## Выходные данные

Массив объектов `Subtask`:

```json
[
  {
    "uuid": "...",
    "taskUuid": "...",
    "name": "Написать тесты",
    "isCompleted": false,
    "position": 0,
    "createdAt": "2026-03-02 10:00:00",
    "updatedAt": "2026-03-02 10:00:00"
  }
]
```

| Поле          | Тип       | Описание                                     |
|---------------|-----------|----------------------------------------------|
| `uuid`        | `string`  | UUID v4, первичный ключ                      |
| `taskUuid`    | `string`  | UUID родительской задачи                     |
| `name`        | `string`  | Название подзадачи                           |
| `isCompleted` | `boolean` | Статус выполнения (по умолчанию `false`)     |
| `position`    | `number`  | Порядковый номер (авто-инкремент внутри задачи) |
| `createdAt`   | `string`  | UTC datetime создания                        |
| `updatedAt`   | `string`  | UTC datetime последнего изменения            |

## Сортировка

По `position ASC`.

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

Если задача не существует — возвращается пустой массив (не ошибка).

## Пример вызова (TypeScript)

```ts
const subtasks = await invoke<Subtask[]>("get_subtasks_by_task", {
  taskUuid: "..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/subtask_cmd.rs` → `get_subtasks_by_task`
- Репозиторий: `src-tauri/src/repository/subtask_repo.rs` → `get_by_task()`
