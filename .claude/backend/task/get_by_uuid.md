# get_task_by_uuid

Получение задачи по UUID.

## Tauri-команда

```
get_task_by_uuid
```

## Входные данные

| Параметр | Тип      | Описание      |
|----------|----------|---------------|
| `uuid`   | `string` | UUID задачи   |

## Выходные данные

Объект `Task` (см. `get_by_project.md`).

## Ошибки

| Ошибка               | Когда                             |
|----------------------|-----------------------------------|
| `NotFound`           | Задача с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом        |

## Пример вызова (TypeScript)

```ts
const task = await invoke<Task>("get_task_by_uuid", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/task_cmd.rs` → `get_task_by_uuid`
- Репозиторий: `src-tauri/src/repository/task_repo.rs` → `get_by_uuid()`
