# update_task

Частичное обновление задачи. Все поля опциональны.

Поля с типом `Option<Option<T>>` позволяют:
- `null` — не трогать поле
- `{ value: null }` — установить поле в `NULL`
- `{ value: "..." }` — установить новое значение

## Tauri-команда

```
update_task
```

## Входные данные

| Параметр           | Тип                        | Описание                                          |
|--------------------|----------------------------|---------------------------------------------------|
| `uuid`             | `string`                   | UUID задачи (обязательно)                         |
| `name`             | `string \| null`           | Новое название                                    |
| `description`      | `string \| null`           | Новое описание                                    |
| `statusUuid`       | `(string \| null) \| null` | Новый статус; `null`-значение сбрасывает в NULL   |
| `priorityUuid`     | `(string \| null) \| null` | Новый приоритет; `null`-значение сбрасывает в NULL|
| `dueDate`          | `(string \| null) \| null` | Новый дедлайн; `null`-значение сбрасывает в NULL  |
| `startDate`        | `(string \| null) \| null` | Новая дата начала                                 |
| `estimatedMinutes` | `(number \| null) \| null` | Новая оценка в минутах                            |
| `position`         | `number \| null`           | Новая позиция                                     |

## Выходные данные

Обновлённый объект `Task`.

## Ошибки

| Ошибка               | Когда                             |
|----------------------|-----------------------------------|
| `NotFound`           | Задача с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом        |

## Пример вызова (TypeScript)

```ts
// Перенести задачу в статус и обнулить приоритет
const updated = await invoke<Task>("update_task", {
  uuid: "...",
  statusUuid: "status-uuid",
  priorityUuid: null,  // null внешний = не трогать поле
  name: null,
  description: null,
  dueDate: null,
  startDate: null,
  estimatedMinutes: null,
  position: null
});
```

## Исходный код

- Команда: `src-tauri/src/commands/task_cmd.rs` → `update_task`
- Репозиторий: `src-tauri/src/repository/task_repo.rs` → `update()`
