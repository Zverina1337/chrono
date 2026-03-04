# delete_priority

Удаление приоритета. Все задачи с этим приоритетом получат `priority_uuid = NULL` (SET NULL).

## Tauri-команда

```
delete_priority
```

## Входные данные

| Параметр | Тип      | Описание          |
|----------|----------|-------------------|
| `uuid`   | `string` | UUID приоритета   |

## Выходные данные

`null` (успех).

## Побочные эффекты

- У всех задач с `priority_uuid = uuid` поле сбрасывается в `NULL` (ON DELETE SET NULL).

## Ошибки

| Ошибка               | Когда                               |
|----------------------|-------------------------------------|
| `NotFound`           | Приоритет с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом          |

## Пример вызова (TypeScript)

```ts
await invoke("delete_priority", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/priority_cmd.rs` → `delete_priority`
- Репозиторий: `src-tauri/src/repository/priority_repo.rs` → `delete()`
