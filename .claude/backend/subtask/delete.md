# delete_subtask

Удаление подзадачи.

## Tauri-команда

```
delete_subtask
```

## Входные данные

| Параметр | Тип      | Описание       |
|----------|----------|----------------|
| `uuid`   | `string` | UUID подзадачи |

## Выходные данные

`null` (успех).

## Ошибки

| Ошибка               | Когда                                |
|----------------------|--------------------------------------|
| `NotFound`           | Подзадача с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом           |

## Пример вызова (TypeScript)

```ts
await invoke("delete_subtask", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/subtask_cmd.rs` → `delete_subtask`
- Репозиторий: `src-tauri/src/repository/subtask_repo.rs` → `delete()`
