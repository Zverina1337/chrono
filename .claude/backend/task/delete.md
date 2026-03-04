# delete_task

Удаление задачи. Каскадно удаляет все подзадачи и записи времени.

## Tauri-команда

```
delete_task
```

## Входные данные

| Параметр | Тип      | Описание    |
|----------|----------|-------------|
| `uuid`   | `string` | UUID задачи |

## Выходные данные

`null` (успех).

## Побочные эффекты

- Удаляются все `subtasks` с `task_uuid = uuid` (ON DELETE CASCADE).
- Удаляются все `time_entries` с `task_uuid = uuid` (ON DELETE CASCADE).
- Удаляются все записи в `task_labels` с `task_uuid = uuid` (ON DELETE CASCADE).

## Ошибки

| Ошибка               | Когда                             |
|----------------------|-----------------------------------|
| `NotFound`           | Задача с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом        |

## Пример вызова (TypeScript)

```ts
await invoke("delete_task", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/task_cmd.rs` → `delete_task`
- Репозиторий: `src-tauri/src/repository/task_repo.rs` → `delete()`
