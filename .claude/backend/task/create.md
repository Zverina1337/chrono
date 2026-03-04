# create_task

Создание новой задачи в проекте.

## Tauri-команда

```
create_task
```

## Входные данные

| Параметр           | Тип               | Обязательно | Описание                            |
|--------------------|-------------------|:-----------:|-------------------------------------|
| `projectUuid`      | `string`          | да          | UUID проекта                        |
| `statusUuid`       | `string \| null`  | нет         | UUID статуса (канбан-колонки)       |
| `priorityUuid`     | `string \| null`  | нет         | UUID приоритета                     |
| `name`             | `string`          | да          | Название задачи                     |
| `description`      | `string`          | да          | Описание (передавай `""` если нет)  |
| `dueDate`          | `string \| null`  | нет         | Дедлайн в формате `YYYY-MM-DD`     |
| `startDate`        | `string \| null`  | нет         | Дата начала `YYYY-MM-DD`            |
| `estimatedMinutes` | `number \| null`  | нет         | Оценка в минутах                    |

## Выходные данные

Объект `Task` (см. `get_by_project.md`).

## Автоматические поля

- `position` — `MAX(position) + 1` среди задач проекта (начинается с 0).
- `uuid` — генерируется как UUID v4.
- `createdAt` / `updatedAt` — текущее UTC-время.

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
const task = await invoke<Task>("create_task", {
  projectUuid: "550e8400-...",
  statusUuid: null,
  priorityUuid: null,
  name: "Реализовать экран задач",
  description: "",
  dueDate: "2026-04-01",
  startDate: null,
  estimatedMinutes: 120
});
```

## Исходный код

- Команда: `src-tauri/src/commands/task_cmd.rs` → `create_task`
- Репозиторий: `src-tauri/src/repository/task_repo.rs` → `create()`
