# delete_time_entry

Удаление записи времени.

## Tauri-команда

```
delete_time_entry
```

## Входные данные

| Параметр | Тип      | Описание              |
|----------|----------|-----------------------|
| `uuid`   | `string` | UUID записи времени   |

## Выходные данные

`null` (успех).

## Ошибки

| Ошибка               | Когда                                    |
|----------------------|------------------------------------------|
| `NotFound`           | Запись с таким UUID не существует        |
| `Ошибка базы данных` | Проблемы с SQLite-запросом               |

## Пример вызова (TypeScript)

```ts
await invoke("delete_time_entry", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/time_entry_cmd.rs` → `delete_time_entry`
- Репозиторий: `src-tauri/src/repository/time_entry_repo.rs` → `delete()`
