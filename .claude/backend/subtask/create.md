# create_subtask

Создание подзадачи.

## Tauri-команда

```
create_subtask
```

## Входные данные

| Параметр   | Тип      | Обязательно | Описание              |
|------------|----------|:-----------:|-----------------------|
| `taskUuid` | `string` | да          | UUID родительской задачи |
| `name`     | `string` | да          | Название подзадачи    |

## Выходные данные

Объект `Subtask` (см. `get_by_task.md`).

## Автоматические поля

- `position` — `MAX(position) + 1` среди подзадач задачи (начинается с 0).
- `isCompleted` — всегда `false` при создании.

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
const subtask = await invoke<Subtask>("create_subtask", {
  taskUuid: "...",
  name: "Написать тесты"
});
```

## Исходный код

- Команда: `src-tauri/src/commands/subtask_cmd.rs` → `create_subtask`
- Репозиторий: `src-tauri/src/repository/subtask_repo.rs` → `create()`
