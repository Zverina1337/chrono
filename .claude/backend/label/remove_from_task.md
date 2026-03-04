# remove_label_from_task

Открепление лейбла от задачи.

## Tauri-команда

```
remove_label_from_task
```

## Входные данные

| Параметр    | Тип      | Описание    |
|-------------|----------|-------------|
| `taskUuid`  | `string` | UUID задачи |
| `labelUuid` | `string` | UUID лейбла |

## Выходные данные

`null` (успех).

## Особенности

- Если связи не существует — ошибки нет (удалено 0 строк — тихий успех).

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
await invoke("remove_label_from_task", {
  taskUuid: "...",
  labelUuid: "..."
});
```

## Исходный код

- Команда: `src-tauri/src/commands/label_cmd.rs` → `remove_label_from_task`
- Репозиторий: `src-tauri/src/repository/label_repo.rs` → `remove_from_task()`
