# get_tasks_by_project

Получение всех задач проекта.

## Tauri-команда

```
get_tasks_by_project
```

## Входные данные

| Параметр      | Тип      | Описание     |
|---------------|----------|--------------|
| `projectUuid` | `string` | UUID проекта |

## Выходные данные

Массив объектов `Task`:

```json
[
  {
    "uuid": "...",
    "projectUuid": "...",
    "statusUuid": "...",
    "priorityUuid": null,
    "name": "Задача",
    "description": "",
    "dueDate": null,
    "startDate": null,
    "estimatedMinutes": null,
    "position": 0,
    "createdAt": "2026-03-02 10:00:00",
    "updatedAt": "2026-03-02 10:00:00"
  }
]
```

| Поле               | Тип               | Описание                                     |
|--------------------|-------------------|----------------------------------------------|
| `uuid`             | `string`          | UUID v4, первичный ключ                      |
| `projectUuid`      | `string`          | UUID проекта                                 |
| `statusUuid`       | `string \| null`  | UUID статуса (канбан-колонки) или `null`     |
| `priorityUuid`     | `string \| null`  | UUID приоритета или `null`                   |
| `name`             | `string`          | Название задачи                              |
| `description`      | `string`          | Описание задачи (по умолчанию `""`)          |
| `dueDate`          | `string \| null`  | Дедлайн в формате `YYYY-MM-DD` или `null`   |
| `startDate`        | `string \| null`  | Дата начала `YYYY-MM-DD` или `null`          |
| `estimatedMinutes` | `number \| null`  | Оценка в минутах или `null`                  |
| `position`         | `number`          | Порядковый номер (авто-инкремент внутри проекта) |
| `createdAt`        | `string`          | UTC datetime создания                        |
| `updatedAt`        | `string`          | UTC datetime последнего изменения            |

## Сортировка

По `position ASC`, затем `created_at ASC`.

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

Если проект не существует — возвращается пустой массив (не ошибка).

## Пример вызова (TypeScript)

```ts
const tasks = await invoke<Task[]>("get_tasks_by_project", {
  projectUuid: "550e8400-..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/task_cmd.rs` → `get_tasks_by_project`
- Репозиторий: `src-tauri/src/repository/task_repo.rs` → `get_by_project()`
